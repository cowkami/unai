use api_client::gpt::Gpt;
use base64::{engine::general_purpose::STANDARD, Engine};
use image::ImageReader;

#[tokio::main]
async fn main() {
    let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");

    let text = "ミーアキャット";
    let image_base64 = llm_client.generate_image(text.to_string()).await.unwrap();
    let image_bytes = STANDARD.decode(image_base64[0].clone()).unwrap();
    let img = ImageReader::new(std::io::Cursor::new(image_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    img.save("output.png").unwrap();
    println!("OK");
}
