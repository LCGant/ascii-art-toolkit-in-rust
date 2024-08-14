// src/filter_type.rs

/*
In this module, you can define and apply various convolution filters.
*/

use image::{DynamicImage, GenericImageView, ImageBuffer, Luma};

pub enum FilterType {
    Sobel,
    Fourier,
}

impl FilterType {

    pub fn apply_filter(&self, img: &DynamicImage) -> DynamicImage {
        // Applies the specified filter based on the FilterType
        match self {
            FilterType::Sobel => Self::apply_sobel_filter(img),
            FilterType::Fourier => Self::apply_fourier_filter(img),
        }
    }

    fn apply_sobel_filter(img: &DynamicImage) -> DynamicImage {
        // Defines the Sobel kernels for edge detection in X and Y directions
        let kernel_x = vec![
            vec![-1.0, 0.0, 1.0],
            vec![-2.0, 0.0, 2.0],
            vec![-1.0, 0.0, 1.0],
        ];
        let kernel_y = vec![
            vec![-1.0, -2.0, -1.0],
            vec![0.0, 0.0, 0.0],
            vec![1.0, 2.0, 1.0],
        ];

        // Apply convolution with both X and Y kernels
        let img_x = Self::apply_convolution_filter(img, &kernel_x);
        let img_y = Self::apply_convolution_filter(img, &kernel_y);
        let (width, height) = img.dimensions();
        let mut output_img = ImageBuffer::new(width, height);

        // Calculate the gradient magnitude and build the output image
        for y in 0..height {
            for x in 0..width {
                let pixel_x = img_x.get_pixel(x, y)[0] as f32;
                let pixel_y = img_y.get_pixel(x, y)[0] as f32;
                let magnitude = (pixel_x.powi(2) + pixel_y.powi(2)).sqrt().clamp(0.0, 255.0) as u8;
                output_img.put_pixel(x, y, Luma([magnitude]));
            }
        }

        DynamicImage::ImageLuma8(output_img)
    }

    fn apply_fourier_filter(img: &DynamicImage) -> DynamicImage {
        // Defines the kernel for sharpening based on the Fourier method
        let kernel = vec![
            vec![0.0, -1.0, 0.0],
            vec![-1.0, 5.0, -1.0],
            vec![0.0, -1.0, 0.0],
        ];

        // Apply convolution with the defined kernel
        Self::apply_convolution_filter(img, &kernel)
    }

    fn apply_convolution_filter(img: &DynamicImage, kernel: &Vec<Vec<f32>>) -> DynamicImage {
        // Applies the convolution filter to the image using the provided kernel
        let (width, height) = img.dimensions();
        let mut output_img = ImageBuffer::new(width, height);
        let kernel_size = kernel.len() as i32;
        let k = kernel_size / 2;
        let gray_img = img.to_luma8();
    
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                let mut sum = 0.0;
    
                // Convolve the kernel over the image
                for ky in -k..=k {
                    for kx in -k..=k {
                        let px = x + kx;
                        let py = y + ky;
    
                        if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                            let pixel = gray_img.get_pixel(px as u32, py as u32)[0] as f32;
                            sum += pixel * kernel[(ky + k) as usize][(kx + k) as usize];
                        }
                    }
                }
    
                // Clamp the result and set the new pixel value
                let new_pixel_value = sum.clamp(0.0, 255.0) as u8;
                output_img.put_pixel(x as u32, y as u32, Luma([new_pixel_value]));
            }
        }
    
        DynamicImage::ImageLuma8(output_img)  
    }
}
