use std::{fs, path::Path};

use image::io::Reader as ImageReader;
use image::{imageops::FilterType, DynamicImage, ImageError};

pub fn resize(
    input: &str,
    output: &str,
    width: i32,
    height: i32,
    percent: f64,
    filter: &str,
) -> i32 {
    match _resize(input, output, width, height, percent, filter) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

fn _resize(
    input: &str,
    output: &str,
    width: i32,
    height: i32,
    percent: f64,
    filter: &str,
) -> Result<(), ImageError> {
    let canon_input = fs::canonicalize(Path::new(input))?;
    let img = ImageReader::open(canon_input.to_str().unwrap())?.decode()?;
    let (new_width, new_height) = get_new_dimensions(&img, width, height, percent);
    let resize_filter = match filter {
        "nearest" => FilterType::Nearest,
        "triangle" => FilterType::Triangle,
        "catmullrom" => FilterType::CatmullRom,
        "gaussian" => FilterType::Gaussian,
        "lanczos3" => FilterType::Lanczos3,
        _ => FilterType::Lanczos3,
    };
    let new_img = img.resize(new_width, new_height, resize_filter);
    let new_file_path = get_new_file_path(input, output)?;
    new_img.save(new_file_path)?;
    Ok(())
}

fn get_new_dimensions(img: &DynamicImage, width: i32, height: i32, percent: f64) -> (u32, u32) {
    let mut new_width: u32 = width as u32;
    let mut new_height: u32 = height as u32;
    if width != 0 || height != 0 {
        if width < 0 {
            new_width = img.width() - width.unsigned_abs();
        }
        if height < 0 {
            new_height = img.height() - height.unsigned_abs();
        }
    } else if percent > 0.0 {
        new_width = (img.width() as f64 * percent / 100.0).round() as u32;
        new_height = (img.height() as f64 * percent / 100.0).round() as u32;
    }
    (new_width, new_height)
}

fn get_new_file_path(input: &str, output: &str) -> Result<String, ImageError> {
    if !output.is_empty() {
        return Ok(String::from(output));
    }
    let canon_input = fs::canonicalize(Path::new(input))?;
    let parent = canon_input.parent().unwrap();
    let new_file_path = format!(
        "{}-out.{}",
        parent
            .join(canon_input.file_stem().unwrap())
            .to_str()
            .unwrap(),
        canon_input.extension().unwrap().to_str().unwrap()
    );
    Ok(new_file_path)
}
