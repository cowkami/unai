use api_client::{
    firestore::MessageRepoImpl,
    gcs::Gcs,
    gpt::Gpt,
    line::{self, Line},
};
use domain::{Actor, Context, Image, ImageMessage, Message, MessageRepo, UserDemand};
use futures::stream::{self, StreamExt};

#[derive(Clone)]
pub struct App {
    pub llm_client: Gpt,
    pub message_client: Line,
    pub storage_client: Gcs,
    pub message_repo: MessageRepoImpl,
}

impl App {
    pub async fn new() -> Result<Self, &'static str> {
        let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");
        let message_client = Line::new().expect("Failed to initialize LINE client");
        let storage_client = Gcs::new()
            .await
            .expect("Failed to initialize Cloud Storage client");
        let message_repo = MessageRepoImpl::new()
            .await
            .expect("Failed to initialize message repository");

        Ok(Self {
            llm_client,
            message_client,
            storage_client,
            message_repo,
        })
    }

    pub async fn conversation(
        &self,
        payload: line::schema::WebhookEvent,
    ) -> Result<(), &'static str> {
        let user_message = self.parse_user_message(payload)?;
        log::info!("User message: {:#?}", user_message);

        self.show_loading_to_user().await?;
        log::trace!("Loading message sent to user");

        let (context, user_demand) = self.detect_user_demand(&user_message).await?;
        log::info!("Context: {:#?}", context);
        log::info!("User demand: {:#?}", user_demand);

        // add context to user message
        let user_message = Message {
            context: Some(context),
            ..user_message.clone()
        };

        // save user message to DB
        self.save_messages(vec![user_message.clone()])
            .await
            .expect("Failed to save user message to DB");

        let history = self
            .message_repo
            // get the recent 10 messages(5 conversations)
            .list_by_user_id(
                user_message.user.id.clone(),
                10,
                // get latest message at the bottom
                domain::OrderDirection::Ascending,
            )
            .await
            .expect("Failed to get messages history");
        log::trace!("History: {:#?}", history);

        let bot_response = match user_demand {
            UserDemand::Chat => self.chat(&user_message, Some(history)).await?,
            UserDemand::CreateImage => self.create_image(&user_message, Some(history)).await?,
        };
        log::info!("Bot message: {:#?}", bot_response);

        // reply chat to LINE
        let message_api_response = self
            .reply(&bot_response, user_message.reply_token)
            .await
            .expect("Failed to send chat to LINE API");
        log::trace!("Message API response: {:#?}", message_api_response);

        // save bot response to DB
        self.save_messages(bot_response)
            .await
            .expect("Failed to save bot response to DB");

        Ok(())
    }

    fn parse_user_message(
        &self,
        payload: line::schema::WebhookEvent,
    ) -> Result<Message, &'static str> {
        let user_message = self
            .message_client
            .get_user_message(payload)
            .expect("Failed to extract user message");

        Ok(user_message)
    }

    async fn show_loading_to_user(&self) -> Result<(), &'static str> {
        self.message_client
            .show_loading()
            .await
            .expect("Failed to show loading");

        Ok(())
    }

    async fn detect_user_demand(
        &self,
        message: &Message,
    ) -> Result<(Context, UserDemand), &'static str> {
        let user_demand = self
            .llm_client
            .detect_demand(message.text.clone())
            .await
            .expect("Failed to detect user demand");

        Ok(user_demand)
    }

    async fn chat(
        &self,
        message: &Message,
        history: Option<Vec<Message>>,
    ) -> Result<Vec<Message>, &'static str> {
        let messages = vec![history.unwrap_or(vec![]), vec![message.clone()]].concat();
        let bot_response = self
            .llm_client
            .chat(messages)
            .await
            .expect("Failed to get LLM response");

        Ok(vec![Message {
            from: Actor::Bot,
            text: bot_response,
            ..message.clone()
        }])
    }

    async fn create_image(
        &self,
        message: &Message,
        history: Option<Vec<Message>>,
    ) -> Result<Vec<Message>, &'static str> {
        let prompt = self.create_image_prompt(message, history).await?;
        let base64_images = self
            .llm_client
            .generate_image(prompt)
            .await
            .expect("Failed to generate image");

        let images: Vec<Image> = base64_images
            .into_iter()
            .map(|image| Image::from_base64(image))
            .collect();

        let previews: Vec<Image> = images
            .clone()
            .into_iter()
            .map(|image| image.to_preview())
            .collect();

        let image_urls = stream::iter(images)
            .then(|img| async {
                self.upload_image(img)
                    .await
                    .expect("Failed to upload image")
            })
            .collect::<Vec<String>>()
            .await;

        let preview_urls = stream::iter(previews)
            .then(|img| async {
                self.upload_image(img)
                    .await
                    .expect("Failed to upload image")
            })
            .collect::<Vec<String>>()
            .await;

        Ok(image_urls
            .into_iter()
            .zip(preview_urls.into_iter())
            .map(|(original, preview)| ImageMessage {
                url: original,
                preview_url: preview,
            })
            .map(|img_msg| Message {
                from: Actor::Bot,
                text: "".to_string(),
                image: Some(img_msg),
                ..message.clone()
            })
            .collect())
    }

    async fn create_image_prompt(
        &self,
        message: &Message,
        history: Option<Vec<Message>>,
    ) -> Result<String, &'static str> {
        let messages = if let Some(history) = history {
            vec![history, vec![message.clone()]].concat()
        } else {
            vec![message.clone()]
        };
        let prompt = self.llm_client.create_image_prompt(messages).await?;

        Ok(prompt)
    }

    async fn upload_image(&self, image: Image) -> Result<String, &'static str> {
        let remote_file_object = self
            .storage_client
            .upload(
                image.file_name(),
                image.to_bytes().expect("Failed to get image bytes"),
            )
            .await
            .expect("Failed to upload image");
        log::trace!("Remote file object: {:#?}", remote_file_object);

        let download_url = self
            .storage_client
            .get_url(remote_file_object)
            .await
            .expect("Falied to get image URL");
        log::trace!("Download URL: {:#?}", download_url);

        Ok(download_url)
    }

    async fn reply(
        &self,
        messages: &Vec<Message>,
        reply_token: Option<String>,
    ) -> Result<reqwest::Response, &'static str> {
        let line_messages = messages
            .iter()
            .cloned()
            .map(|message| message.into())
            .collect();

        // reply messages to messaging app API
        let response = if let Some(reply_token) = reply_token {
            self.message_client
                .reply_messages(line_messages, reply_token)
                .await
        } else {
            Err("Reply token is required")
        };

        response
    }

    async fn save_messages(&self, messages: Vec<Message>) -> Result<(), &'static str> {
        self.message_repo.save(messages).await
    }
}
