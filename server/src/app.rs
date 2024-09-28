use domain::user::{User, UserDemand};
use service::gpt::Gpt;
use service::{line, line::Line};

#[derive(Clone)]
pub struct App {
    pub llm_client: Gpt,
    pub message_client: Line,
}

impl App {
    pub fn new() -> Result<Self, &'static str> {
        let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");
        let message_client = Line::new().expect("Failed to initialize LINE client");
        Ok(Self {
            llm_client,
            message_client,
        })
    }

    pub async fn conversation(
        &self,
        payload: line::schema::WebhookEvent,
    ) -> Result<(), &'static str> {
        let user_message = self
            .message_client
            .get_user_message(payload)
            .expect("Failed to extract user message");

        log::info!("User message: {:#?}", user_message);

        // show loading to user
        self.message_client
            .show_loading()
            .await
            .expect("Failed to show loading");

        // detect user purpose
        let user_demand = self
            .llm_client
            .detect_demand(user_message.text.clone())
            .await
            .expect("Failed to detect user demand");

        println!("User demand: {:#?}", user_demand);

        // save the image to local storage
        // make the preview image to local storage
        // the image send to GCS and return the URL
        // send the URL to LINE as image message
        let user_message_text = user_message.text.clone();
        let bot_response = match user_demand {
            UserDemand::Chat => self.chat(user_message_text).await,
            UserDemand::CreateImage => self.create_image(user_message_text).await,
        }
        .expect("Failed to get bot response");

        log::info!("Bot message: {:#?}", bot_response);

        // reply chat to LINE
        self.message_client
            .reply(bot_response, user_message.reply_token.unwrap())
            .await
            .expect("Failed to send chat to LINE API");

        Ok(())
    }

    async fn chat(&self, chat: String) -> Result<String, &'static str> {
        let bot_response = self
            .llm_client
            .chat(chat)
            .await
            .expect("Failed to get LLM response");
        Ok(bot_response)
    }

    async fn create_image(&self, text: String) -> Result<String, &'static str> {
        let image = self.llm_client.generate_image(text).await;

        println!("Image created: {:#?}", image);

        Ok("Image created".to_string())
    }
}
