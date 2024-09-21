use chatbot::{gpt::Gpt, line::Line};

#[derive(Clone)]
pub struct AppContext {
    pub llm_client: Gpt,
    pub message_client: Line,
}

impl AppContext {
    pub fn new() -> Result<Self, &'static str> {
        let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");
        let message_client = Line::new().expect("Failed to initialize LINE client");
        Ok(Self {
            llm_client,
            message_client,
        })
    }
}
