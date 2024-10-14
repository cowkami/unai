pub mod schema;

use domain::{Context, UserDemand};
use schema::*;
use serde_json::json;
use std::env;

#[derive(Clone)]
pub struct Gpt {
    api_key: String,
    image_config: ImageConfig,
}

impl Gpt {
    pub fn new() -> Result<Self, &'static str> {
        let api_key =
            env::var("OPENAI_API_KEY").expect("Please set the OPENAI_API_KEY environment variable");

        let image_config = ImageConfig::new().expect("Failed to create ImageConfig");

        Ok(Self {
            api_key,
            image_config,
        })
    }

    pub async fn completions(
        &self,
        request: CompletionsRequest,
    ) -> Result<CompletionsResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        response.json().await
    }

    pub async fn generate_image(&self, prompt: String) -> Result<Vec<String>, &'static str> {
        let request = GenerateImageRequest {
            model: self.image_config.model.to_string(),
            prompt,
            n: self.image_config.count,
            size: self.image_config.size.to_num(),
            response_format: Some(GenImageResponseFormat::B64Json),
        };
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/images/generations")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .expect("Failed to send generate image request");

        let response: GenerateImageResponse = response
            .json()
            .await
            .expect("Failed to parse GenerateImageResponse");

        let images: Vec<String> = response
            .data
            .into_iter()
            .map(|image| image.b64_json)
            .collect();

        Ok(images)
    }

    pub async fn chat(&self, messages: Vec<domain::Message>) -> Result<String, &'static str> {
        let messages = messages
            .into_iter()
            .map(|message| Message {
                role: message.from.into(),
                content: message.text,
            })
            .collect::<Vec<Message>>();

        let request = CompletionsRequest {
            model: "gpt-4o".to_string(),
            messages,
            temperature: Some(0.7),
            response_format: None,
        };
        let response = self
            .completions(request)
            .await
            .expect("Failed to get chat response");

        let text = response.choices[0].message.content.clone();
        Ok(text)
    }

    pub async fn detect_demand(
        &self,
        chat: String,
    ) -> Result<(Context, UserDemand), reqwest::Error> {
        let response_format = ResponseFormat::new(
            "user_demand".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "context": {
                        "type": "string"
                    },
                    "user_demand": {
                        "type": "string",
                        "enum": ["Chat", "CreateImage"],
                    },
               },
                "required": ["context", "user_demand"],
                "additionalProperties": false,
            }),
        );
        let request = CompletionsRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![
                Message {
                    role: Role::System,
                    content: "You are an expert at detecting user demand. \n\
                        Describe the user's demand as a short title for context field.\n\
                        AND Choose the most appropriate label for context from the following options:\n\
                        - Chat\n\
                        - CreateImage"
                        .to_string(),
                },
                Message {
                    role: Role::User,
                    content: chat,
                },
            ],
            temperature: Some(0.0),
            response_format: Some(response_format),
        };

        let response = self.completions(request).await?;
        let content = response.choices[0].message.content.clone();
        let user_demand: schema::UserDemand =
            serde_json::from_str(&content).expect("Failed to parse user demand");

        let context = Context::new(user_demand.context);
        let user_demand =
            UserDemand::try_from(user_demand.user_demand).expect("Invalid user demand");

        Ok((context, user_demand))
    }

    pub async fn create_image_prompt(
        &self,
        messages: Vec<domain::Message>,
    ) -> Result<String, &'static str> {
        let system_message = Message {
            role: Role::System,
            content: "You are an expert at creating image prompts.
            You will be given a chat history.
            You need to create an image prompt mainly for the latest message.
            But you can also use previous messages to create the prompt.
            "
            .to_string(),
        };
        let messages = messages
            .into_iter()
            .map(|message| Message {
                role: message.from.into(),
                content: message.text,
            })
            .collect::<Vec<Message>>();
        let request = CompletionsRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![vec![system_message], messages].concat(),
            temperature: Some(0.7),
            response_format: None,
        };
        let response = self
            .completions(request)
            .await
            .expect("Failed to create image prompt");
        let text = response.choices[0].message.content.clone();
        Ok(text)
    }
}

#[derive(Clone)]
struct ImageConfig {
    model: ImageModel,
    size: ImageSize,
    count: i64,
}

impl ImageConfig {
    fn new() -> Result<Self, &'static str> {
        let image_model = env::var("GENERATE_IMAGE_MODEL")
            .expect("Please set the IMAGE_MODEL environment variable")
            .try_into()?;

        let image_size = env::var("GENERATE_IMAGE_SIZE")
            .expect("Please set the IMAGE_SIZE environment variable")
            .try_into()?;

        let image_count = env::var("GENERATE_IMAGE_COUNT")
            .expect("Please set the N_IMAGE environment variable")
            .parse()
            .expect("Failed to parse N_IMAGE");

        Ok(ImageConfig {
            model: image_model,
            size: image_size,
            count: image_count,
        })
    }
}

#[derive(Clone)]
enum ImageModel {
    DallE2,
    DallE3,
}

impl TryInto<ImageModel> for String {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageModel, Self::Error> {
        match self.as_str() {
            "dall-e-2" => Ok(ImageModel::DallE2),
            "dall-e-3" => Ok(ImageModel::DallE3),
            _ => Err("Invalid image model"),
        }
    }
}

impl ToString for ImageModel {
    fn to_string(&self) -> String {
        match self {
            Self::DallE2 => "dall-e-2".to_string(),
            Self::DallE3 => "dall-e-3".to_string(),
        }
    }
}

#[derive(Clone)]
enum ImageSize {
    Small,     // 256x256
    Medium,    // 512x512
    Large,     // 1024x1024
    Landscape, // 1792x1024
    Portrait,  // 1024x1792
}

impl ImageSize {
    fn to_num(&self) -> String {
        match self {
            Self::Small => "256x256".to_string(),
            Self::Medium => "512x512".to_string(),
            Self::Large => "1024x1024".to_string(),
            Self::Landscape => "1792x1024".to_string(),
            Self::Portrait => "1024x1792".to_string(),
        }
    }
}

impl TryInto<ImageSize> for String {
    type Error = &'static str;

    fn try_into(self) -> Result<ImageSize, Self::Error> {
        Ok(match self.as_str() {
            "small" => ImageSize::Small,
            "medium" => ImageSize::Medium,
            "large" => ImageSize::Large,
            "landscape" => ImageSize::Landscape,
            "portrait" => ImageSize::Portrait,
            _ => panic!("Invalid image size"),
        })
    }
}
