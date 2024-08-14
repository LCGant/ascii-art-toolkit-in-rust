mod image_utils;
mod ascii_art;
mod export_art;
mod filter_type;
mod char_sets;

use export_art::save_ascii_art_to_file;
use filter_type::FilterType;
use char_sets::CharSet;

fn main() {
    // Define the path to the input image.
    let input_path = r"path/to/your/directory";
    
    // Create an instance of the AsciiArtGenerator.
    let mut ascii_gen = ascii_art::AsciiArtGenerator::new();    

    // Load the image. If there's an error, print it and exit.
    if let Err(e) = ascii_gen.load_image(input_path) {
        eprintln!("Error loading image: {}", e);
        return;
    }

    // Set character set and disable colors for the ASCII art.
    ascii_gen.set_charset(CharSet::MonoBlock);
    ascii_gen.disable_colors();
    
    // Generate ASCII art with dimensions 100x100 and a Sobel filter.
    match ascii_gen.generate_ascii_art(100, 100, Some(FilterType::Sobel)) {
        Ok(ascii_art) => {
            println!("{}", ascii_art);
            if let Err(e) = save_ascii_art_to_file(&ascii_art) {
                eprintln!("Error saving ASCII art: {}", e);
            }
        }
        Err(e) => eprintln!("Error generating ASCII: {}", e),
    }
}
