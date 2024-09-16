use std::env;

pub struct OpenAiApi {
    api_key: String,
    project_id: String,
}

impl OpenAiApi {
    pub fn new() -> Result<Self, &'static str> {
        let api_key =
            env::var("OPENAI_API_KEY").expect("Please set the OPENAI_API_KEY environment variable");
        let project_id = env::var("OPENAI_PROJECT_ID")
            .expect("Please set the OPENAI_PROJECT_ID environment variable");
        Ok(Self {
            api_key,
            project_id,
        })
    }
}
