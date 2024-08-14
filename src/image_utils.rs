// src/image_utils.rs

use image::{DynamicImage};

pub struct ImageReaderWrapper {
    img: Option<DynamicImage>,
}

impl ImageReaderWrapper {
    pub fn new() -> Self {
        ImageReaderWrapper { img: None }
    }

    pub fn load_image(&mut self, path: &str) -> Result<(), image::ImageError> {
        self.img = Some(image::open(path)?);
        Ok(())
    }

    pub fn get_image(&self) -> Option<&DynamicImage> {
        self.img.as_ref()
    }
}
