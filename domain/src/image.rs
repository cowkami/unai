use base64::{engine::general_purpose::STANDARD, Engine};
use image::ImageReader;
use std::path::Path;

#[derive(Debug)]
pub struct Image {
    name: String,
    source: Option<Source>,
    data: Option<ImageData>,
}

impl Image {
    pub fn new(name: String, source: Option<Source>, data: Option<ImageData>) -> Self {
        Self { name, source, data }
    }

    // pub fn save(&self) {
    //     let image_bytes = self
    //         .data
    //         .and_then(|data| match data {
    //             ImageData::Base64(base64) => Some(STANDARD.decode(base64).unwrap()),
    //             ImageData::Bytes(bytes) => Some(bytes),
    //             _ => {
    //                 println!("Invalid image data");
    //                 None
    //             }
    //         })
    //         .unwrap();

    //     let img = ImageReader::new(std::io::Cursor::new(image_bytes))
    //         .with_guessed_format()
    //         .unwrap()
    //         .decode()
    //         .unwrap();

    //     img.save(Path::new(&self.name)).unwrap();
    // }
}

#[derive(Debug)]
pub enum ImageData {
    Base64(String),
    Bytes(Vec<u8>),
}

#[derive(Debug)]
pub enum Source {
    Path(String),
    Url(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image() {
        let image = Image {
            name: "test".to_string(),
            source: None,
            data: None,
        };

        assert_eq!(image.name, "test");
    }
}
