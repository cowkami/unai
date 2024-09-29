use domain::image::Image;
use domain::message::Message;
use domain::user::UserDemand;
use futures::stream::{self, StreamExt};
use service::{
    gcs::Gcs,
    gpt::Gpt,
    line::{self, schema::Message as LineMessage, Line},
};

#[derive(Clone)]
pub struct App {
    pub llm_client: Gpt,
    pub message_client: Line,
    pub storage_client: Gcs,
}

impl App {
    pub async fn new() -> Result<Self, &'static str> {
        let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");
        let message_client = Line::new().expect("Failed to initialize LINE client");
        let storage_client = Gcs::new()
            .await
            .expect("Failed to initialize Cloud Storage client");
        Ok(Self {
            llm_client,
            message_client,
            storage_client,
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

        let user_demand = self.detect_user_demand(user_message.text.clone()).await?;
        log::info!("User demand: {:#?}", user_demand);

        let user_message_text = user_message.text.clone();
        let bot_response = match user_demand {
            UserDemand::Chat => self.chat(user_message_text).await?,
            UserDemand::CreateImage => self.create_image(user_message_text).await?,
        };
        log::info!("Bot message: {:#?}", bot_response);

        // reply chat to LINE
        let message_api_response = self
            .message_client
            .reply_messages(bot_response, user_message.reply_token.unwrap())
            .await
            .expect("Failed to send chat to LINE API");
        log::trace!("Message API response: {:#?}", message_api_response);

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

    async fn detect_user_demand(&self, chat: String) -> Result<UserDemand, &'static str> {
        let user_demand = self
            .llm_client
            .detect_demand(chat)
            .await
            .expect("Failed to detect user demand");

        Ok(user_demand)
    }

    async fn chat(&self, chat: String) -> Result<Vec<LineMessage>, &'static str> {
        let bot_response = self
            .llm_client
            .chat(chat)
            .await
            .expect("Failed to get LLM response");

        Ok(vec![LineMessage::text(bot_response, None)])
    }

    async fn create_image(&self, text: String) -> Result<Vec<LineMessage>, &'static str> {
        let base64_images = self
            .llm_client
            .generate_image(text)
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
            .map(|(original, preview)| LineMessage::image(original, preview))
            .collect())
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
}
