# ASCII Art Generator

This project is a Rust-based ASCII art generator that converts images into ASCII art. It supports various convolution filters, different character sets, and colored ASCII art.

## Features

- **Image to ASCII Conversion**: Converts images to ASCII art with various character sets and colors.
- **Convolution Filters**: Apply different types of convolution filters to modify the image before converting it to ASCII.
- **Support for Different Character Sets**: Use various character sets to represent different shades and textures in the ASCII art.
- **Colored ASCII Art**: Generate ASCII art with color support, provided the image is colored.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/LCGant/ascii-art-toolkit-in-rust.git
   
2. Ensure you have Rust installed. You can install Rust from the [official website](https://www.rust-lang.org/).


3. Build the project:
    ```bash
    cargo build

## Usage

1. Modify the main.rs file to specify the image path and desired ASCII art parameters.

2. Run the project:
    ```bash
    cargo run

This will read the specified image, generate the ASCII art, and save it to a text file named ascii_art.txt.

## Example

```rust
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
```

## Contributing
Feel free to contribute to the project by submitting issues or pull requests. Please follow the code style and guidelines provided in the project's repository.

## License

This project is licensed under the CC0 1.0 Universal License. You can use, modify, and distribute this code without restrictions. For more details, see [CC0 1.0 Universal License](https://creativecommons.org/publicdomain/zero/1.0/).
