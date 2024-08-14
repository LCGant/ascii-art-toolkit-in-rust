// src/ascii_art.rs

/*
This module contains the logic to transform an image into ASCII art.
*/

use crate::image_utils::ImageReaderWrapper;
use image::{imageops, DynamicImage, GenericImageView};
use colored::*;
use std::error::Error;

use crate::filter_type::FilterType;
use crate::char_sets::{CharSets, CharSet};

pub struct AsciiArtGenerator {
    img: Option<DynamicImage>,
    char_sets: CharSets,
    use_colors: bool,
}

impl AsciiArtGenerator {
    pub fn new() -> Self {
        let mut char_sets = CharSets::new();
        char_sets.set_charset(CharSet::Point);

        AsciiArtGenerator { 
            img: None, 
            char_sets,
            use_colors: false,
        }
    }

    pub fn set_charset(&mut self, char_set: CharSet) {
        self.char_sets.set_charset(char_set);
    }

    pub fn enable_colors(&mut self) {
        self.use_colors = true;
    }

    pub fn disable_colors(&mut self) {
        self.use_colors = false;
    }

    pub fn load_image(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut reader = ImageReaderWrapper::new();
        reader.load_image(path)?;
        self.img = reader.get_image().cloned();
        Ok(())
    }

    pub fn generate_ascii_art(
        &self,
        max_width: u32,
        max_height: u32,
        filter_type: Option<FilterType>,
    ) -> Result<String, Box<dyn Error>> {
        if let Some(img) = &self.img {
            // Apply filter if provided, otherwise use the original image
            let processed_img = if let Some(filter) = filter_type {
                filter.apply_filter(img)
            } else {
                img.clone()
            };
    
            // Calculate new dimensions maintaining the aspect ratio
            let (width, height) = processed_img.dimensions();
            let aspect_ratio = width as f32 / height as f32;
            let (new_width, new_height) = if width > height {
                (max_width, (max_width as f32 / aspect_ratio).round() as u32)
            } else {
                ((max_height as f32 * aspect_ratio).round() as u32, max_height)
            };
    
            // Resize image to fit within the specified dimensions
            let resized_img = imageops::resize(&processed_img, new_width, new_height, imageops::FilterType::Nearest);
    
            let mut ascii_art = String::new();
            let contains_space = self.char_sets.ascii_chars.contains(&' ');
    
            for y in 0..new_height {
                for x in 0..new_width {
                    let pixel = resized_img.get_pixel(x, y);
                    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
                    // Calculate brightness using the luminosity method
                    let brightness = (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114) / 255.0;
    
                    // Choose ASCII character based on brightness
                    let ascii_char = if contains_space && brightness > 0.8 {
                        ' '
                    } else {
                        self.pixel_to_ascii(r)
                    };
    
                    // Determine if color should be applied
                    let color = if self.use_colors {
                        Some(self.pixel_to_color(r, g, b))
                    } else {
                        None
                    };
    
                    // Append colored or plain ASCII character to the output
                    if let Some(color) = color {
                        ascii_art.push_str(&format!("{}", ascii_char.to_string().truecolor(color.0, color.1, color.2)));
                    } else {
                        ascii_art.push(ascii_char);
                    }
                }
                ascii_art.push('\n');
            }
    
            Ok(ascii_art)
        } else {
            Err("No image loaded.".into())
        }
    }
     
    fn pixel_to_ascii(&self, pixel_value: u8) -> char {
        // Map pixel value to ASCII character based on brightness
        let scale = self.char_sets.ascii_chars.len() as u8;
        let index = (pixel_value as f32 / 255.0 * (scale as f32 - 1.0)) as usize;
        self.char_sets.ascii_chars[index]
    }

    fn pixel_to_color(&self, r: u8, g: u8, b: u8) -> (u8, u8, u8) {
        // Return the color values for use in colored ASCII output
        (r, g, b)
    }
}
