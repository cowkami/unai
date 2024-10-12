pub use google_cloud_storage::http::objects::Object as GcsObject;
use google_cloud_storage::{
    client::{google_cloud_auth::credentials, Client, ClientConfig},
    http::objects::upload::{Media, UploadObjectRequest, UploadType},
    sign::{SignedURLMethod, SignedURLOptions},
};
use mockall::automock;
use std::env;

#[automock]
pub trait FileStorageClient {
    async fn upload(&self, file_name: String, data: Vec<u8>) -> Result<GcsObject, &'static str>;
    async fn get_url(&self, uploaded: GcsObject) -> Result<String, &'static str>;
}

#[derive(Clone)]
pub struct Gcs {
    client: Client,
    bucket: String,
}

impl Gcs {
    pub async fn new() -> Result<Self, &'static str> {
        let bucket =
            env::var("GCS_BUCKET").expect("Please set the GCS_BUCKET environment variable");

        let google_application_credentials = env::var("GOOGLE_APPLICATION_CREDENTIALS")
            .expect("Please set the GOOGLE_APPLICATION_CREDENTIALS environment variable");

        let credentials =
            credentials::CredentialsFile::new_from_file(google_application_credentials)
                .await
                .expect("Failed to get GCS credentials");

        let config = ClientConfig::default()
            .with_credentials(credentials)
            .await
            .expect("Failed to initialize GCS client config");
        let client = Client::new(config);

        Ok(Gcs { client, bucket })
    }
}

impl FileStorageClient for Gcs {
    async fn upload(&self, file_name: String, data: Vec<u8>) -> Result<GcsObject, &'static str> {
        let upload_type = UploadType::Simple(Media::new(file_name));
        let uploaded = self
            .client
            .upload_object(
                &UploadObjectRequest {
                    bucket: self.bucket.clone(),
                    ..Default::default()
                },
                data,
                &upload_type,
            )
            .await
            .expect("Failed to upload object");

        Ok(uploaded)
    }

    async fn get_url(&self, uploaded: GcsObject) -> Result<String, &'static str> {
        let options = SignedURLOptions {
            method: SignedURLMethod::GET,
            expires: std::time::Duration::from_secs(600),
            ..Default::default()
        };

        let url_for_download = self
            .client
            .signed_url(&uploaded.bucket, &uploaded.name, None, None, options)
            .await
            .expect("Failed to get signed URL");

        Ok(url_for_download)
    }
}
