use api_client::gpt::Gpt;
use domain::image::Image;

#[tokio::main]
async fn main() {
    let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");

    let text = "ミーアキャット";
    let image_base64 = llm_client.generate_image(text.to_string()).await.unwrap();

    let image = Image::from_base64(image_base64[0].clone());
    image.save("./".to_string()).unwrap();

    let image = Image::from_base64(image_base64[0].clone()).to_preview();
    image.save("./".to_string()).unwrap();

    println!("OK");
}
