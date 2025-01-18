use wasm_bindgen::prelude::wasm_bindgen;
use crate::models::Pay;

pub mod encoder;
pub mod models;
pub mod qr;

#[wasm_bindgen]
pub fn encode_pay_xml_to_svg(xml: &str) -> String {
    let pay: Pay = quick_xml::de::from_str(&xml).expect("unable to decode XML file");
    let encoded = encoder::encode(&pay);
    let svg = qr::create_pay_svg(&encoded, qr::Theme::default());
    String::from_utf8(svg).unwrap()
}
