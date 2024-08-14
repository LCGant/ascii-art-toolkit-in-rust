//src/export_art.rs

/*
Area for exporting artwork to .txt. In the future,
 I want to add the option to export as PNG.
*/

use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn save_ascii_art_to_file(
    ascii_art: &str,
) -> Result<(), Box<dyn Error>> {
    let path = "ascii_art.txt";
    let mut file = File::create(path)?;
    file.write_all(ascii_art.as_bytes())?;
    Ok(())
}
