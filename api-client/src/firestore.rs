use chrono::prelude::*;
use domain::{Actor, Message, MessageRepo, User};
use firestore::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct MessageRepoImpl {
    db: FirestoreDb,
}

impl MessageRepoImpl {
    pub async fn new() -> Result<Self, &'static str> {
        let project_id = std::env::var("GOOGLE_PROJECT_ID").expect("GOOGLE_PROJECT_ID is not set");
        let database_id = std::env::var("FIRESTORE_DB_ID").expect("FIRESTORE_DB_ID is not set");
        let credentials = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
            .expect("GOOGLE_APPLICATION_CREDENTIALS is not set");

        Ok(Self {
            db: FirestoreDb::with_options_service_account_key_file(
                FirestoreDbOptions::new(project_id).with_database_id(database_id),
                credentials.into(),
            )
            .await
            .expect("Failed to initilize Firestore client"),
        })
    }
}

impl MessageRepo for MessageRepoImpl {
    async fn save(&self, messages: Vec<Message>) -> Result<(), &'static str> {
        let message: MessageDocument = messages[0]
            .clone()
            .try_into()
            .expect("Invalid message to insert into DB");

        let _: MessageDocument = self
            .db
            .fluent()
            .insert()
            .into("messages")
            .document_id(Uuid::new_v4().to_string())
            .object(&message)
            .execute()
            .await
            .expect("Failed to save messages to Firestore");

        Ok(())
    }

    async fn list_by_user_id(
        &self,
        user_id: String,
        limit: u32,
    ) -> Result<Vec<Message>, &'static str> {
        let messages = self
            .db
            .fluent()
            .select()
            .from("messages")
            .filter(|q| q.field("userId").eq(&user_id))
            .order_by([("createdTime", FirestoreQueryDirection::Descending)])
            .limit(limit)
            .obj::<MessageDocument>()
            .query()
            .await
            .map_err(|_| "Failed to get messages from Firestore")?;

        Ok(messages
            .into_iter()
            .map(|doc| doc.try_into().expect("Failed to parse message"))
            .collect())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MessageDocument {
    user_id: String,
    from: Sender,
    text: String,
    context_id: String,
    context_name: String,

    #[serde(with = "firestore::serialize_as_timestamp")]
    created_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Sender {
    User,
    Bot,
}

impl From<Actor> for Sender {
    fn from(actor: Actor) -> Self {
        match actor {
            Actor::User => Self::User,
            Actor::Bot => Self::Bot,
        }
    }
}

impl From<Sender> for Actor {
    fn from(sender: Sender) -> Self {
        match sender {
            Sender::User => Actor::User,
            Sender::Bot => Actor::Bot,
        }
    }
}

impl TryFrom<Message> for MessageDocument {
    type Error = &'static str;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        let context = message.context.expect("Context is required");

        Ok(Self {
            user_id: message.user.id,
            from: message.from.into(),
            text: message.text.clone(),
            context_id: context.id.to_string(),
            context_name: context.name,
            created_time: Utc::now(),
        })
    }
}

impl TryFrom<MessageDocument> for Message {
    type Error = &'static str;

    fn try_from(doc: MessageDocument) -> Result<Self, Self::Error> {
        Ok(Message {
            user: User {
                id: doc.user_id.clone(),
            },
            from: doc.from.into(),
            text: doc.text,
            context: None,
            reply_token: None,
            image: None,
        })
    }
}
