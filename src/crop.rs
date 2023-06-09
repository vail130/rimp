use std::{fs, path::Path};

use image::io::Reader as ImageReader;
use image::ImageError;

use rimp::get_new_file_path;

pub fn crop(input: &str, output: &str, x: u32, y: u32, width: u32, height: u32) -> i32 {
    match _crop(input, output, x, y, width, height) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

fn _crop(
    input: &str,
    output: &str,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<(), ImageError> {
    let canon_input = fs::canonicalize(Path::new(input))?;
    let img = ImageReader::open(canon_input.to_str().unwrap())?.decode()?;
    let new_img = img.crop_imm(x, y, width, height);
    new_img.save(get_new_file_path(input, output)?)?;
    Ok(())
}
