use std::io::Cursor;

use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, ImageBuffer, Rgba};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // #[wasm_bindgen(js_namespace = console)]
    // fn log(s: &str);
}

#[wasm_bindgen]
pub fn process(url: &str,filter:&str) -> String {
    let img = decode_image(url);

    let processed_img = match filter{
        "rotate90" => Some(image::imageops::rotate90(&img)),
        "blur" => Some(image::imageops::blur(&img,5.0)),
        _ => None
    };

    encode_image(&processed_img.unwrap())
}

fn encode_image(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
    let mut buf: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)
        .unwrap();
    let encode = general_purpose::STANDARD.encode(&buf);
    encode
}

fn decode_image(base_64: &str) -> DynamicImage {
    let decode = general_purpose::STANDARD.decode(base_64).unwrap();
    let dynamic_img =
        image::load_from_memory_with_format(&decode, image::ImageFormat::Png).unwrap();
    dynamic_img
}
