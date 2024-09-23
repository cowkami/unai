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

        // send chat
        let bot_response = self
            .llm_client
            .chat(user_message.text)
            .await
            .expect("Failed to send chat to OpenAI API");

        log::info!("Bot message: {:#?}", bot_response);

        // reply chat to LINE
        self.message_client
            .reply(bot_response, user_message.reply_token.unwrap())
            .await
            .expect("Failed to send chat to LINE API");

        Ok(())
    }
}
