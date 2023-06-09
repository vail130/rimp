use std::{fs, path::Path};

use image::ImageError;

pub fn get_new_file_path(input: &str, output: &str) -> Result<String, ImageError> {
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
