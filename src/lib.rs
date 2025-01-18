use wasm_bindgen::prelude::wasm_bindgen;
use crate::models::Pay;

pub mod encoder;
pub mod models;
pub mod qr;

#[wasm_bindgen]
pub fn encode_to_svg(source: &str) -> String {
    let pay: Pay = models::try_deserialize_pay(&source);
    let encoded = encoder::encode(&pay);
    let svg = qr::create_pay_svg(&encoded, qr::Theme::default());
    String::from_utf8(svg).unwrap()
}

#[wasm_bindgen]
pub fn encode_to_png(source: &str, size: u32) -> String {
    let pay: Pay = models::try_deserialize_pay(&source);
    let encoded = encoder::encode(&pay);
    let svg = qr::create_pay_svg(&encoded, qr::Theme::default());
    qr::to_base64_png(&svg, size)
}

#[wasm_bindgen]
pub fn encode_to_jpeg(source: &str, size: u32, quality: u8) -> String {
    let pay: Pay = models::try_deserialize_pay(&source);
    let encoded = encoder::encode(&pay);
    let svg = qr::create_pay_svg(&encoded, qr::Theme::default());
    qr::to_base64_jpeg(&svg, size, quality)
}
