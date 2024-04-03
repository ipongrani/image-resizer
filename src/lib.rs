use image::{GenericImageView};
use image::imageops;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn resize_image(width: Option<u32>, height: Option<u32>, input_image_data: Vec<u8>) -> Vec<u8> {
    // Decode the input image data into a DynamicImage
    let img = image::load_from_memory(&input_image_data).unwrap();

    let use_width = width.unwrap_or(img.width());
    let use_height = height.unwrap_or(img.height());

    // Resize the image
    let resized_img = img.resize(use_width, use_height, imageops::FilterType::Lanczos3);

    // Convert the resized image to RGBA format
    let rgba_img = resized_img.to_rgba8();

    // Return the resized image data as a Vec<u8>
    rgba_img.into_raw()
}