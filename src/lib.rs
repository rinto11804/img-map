use base64::{engine::general_purpose, Engine as _};
use image::ImageOutputFormat;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlImageElement};

#[wasm_bindgen]
extern "C" {
    // #[wasm_bindgen(js_namespace = console)]
    // fn log(s: &str);
}

#[wasm_bindgen]
pub fn grayscale() -> Result<(), wasm_bindgen::JsValue> {
    let doc = window().unwrap().document().unwrap();
    let img = doc
        .get_element_by_id("img")
        .unwrap()
        .dyn_into::<HtmlImageElement>()?;

    let image = image::load_from_memory(&img.src().as_bytes()).unwrap();
    // let grayscale_img = image.grayscale();

    let mut image_data: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let res_base64 = general_purpose::STANDARD.encode(&image_data);
    let src = format!("data:image/png;base64,{}", res_base64);
   img.set_src(&src);

    let body = doc.body().unwrap();
    body.append_child(&img)?;
    Ok(())
}
