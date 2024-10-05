use base64::{engine::general_purpose::STANDARD, Engine};
use image::{load_from_memory, ImageBuffer, ImageFormat, Rgba};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Image {
    pub id: Uuid,
    pub path: Option<PathBuf>,
    pub data: ImageData,
    pub is_preview: bool,
}

impl Image {
    pub fn default(data: ImageData) -> Self {
        Self {
            id: Uuid::new_v4(),
            path: None,
            data,
            is_preview: false,
        }
    }
    pub fn from_base64(base64: String) -> Self {
        Self::default(ImageData::Base64(base64))
    }

    pub fn file_name(&self) -> String {
        format!(
            "{}{}.png",
            if self.is_preview { "preview_" } else { "" },
            &self.id
        )
    }

    pub fn save(&self, output_dir: String) -> Result<Self, &'static str> {
        let bytes = self.to_bytes().expect("Failed to get image bytes");
        let img = load_from_memory(&bytes).expect("Failed to decode image");

        let file_name = self.file_name();
        let path = Path::new(&output_dir).join(file_name);
        img.save_with_format(path.clone(), ImageFormat::Png)
            .expect("Failed to save image");

        Ok(Self {
            id: self.id,
            path: Some(path),
            data: ImageData::Bytes(bytes),
            is_preview: self.is_preview,
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, &'static str> {
        match &self.data {
            ImageData::Base64(base64) => {
                Ok(STANDARD.decode(base64).expect("Failed to decode base64"))
            }
            ImageData::Bytes(bytes) => Ok(bytes.clone()),
        }
    }

    pub fn to_preview(&self) -> Self {
        let preview = self.resize(512, 512).expect("Failed to resize image");
        Self {
            id: preview.id,
            path: preview.path,
            data: preview.data,
            is_preview: true,
        }
    }

    fn resize(&self, width: u32, height: u32) -> Result<Self, &'static str> {
        let bytes = self.to_bytes()?;
        let img = image::load_from_memory(&bytes).expect("Failed to load image for resizing");

        let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);

        // Convert the resized image to RGBA
        let rgba_image: ImageBuffer<Rgba<u8>, Vec<u8>> = resized.to_rgba8();

        // Encode the image to PNG
        let mut png_data = Vec::new();
        let mut cursor = Cursor::new(&mut png_data);
        rgba_image
            .write_to(&mut cursor, ImageFormat::Png)
            .expect("Failed to encode resized image to PNG");

        Ok(Self {
            id: self.id,
            path: None,
            data: ImageData::Bytes(png_data),
            is_preview: false,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ImageData {
    Base64(String),
    Bytes(Vec<u8>),
}
