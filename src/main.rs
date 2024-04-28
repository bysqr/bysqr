use std::fs;

mod models;
mod encoder;
use qrcode::QrCode;
// use image::Luma;
use qrcode::render::unicode;

use crate::models::Pay;

fn main() {
    let xml = fs::read_to_string("/Users/peter/code/bysqr/test.xml").unwrap();

    let pay: Pay = quick_xml::de::from_str(&xml).unwrap();

    let encoded = encoder::encode(&pay);

    let code = QrCode::new(encoded.as_bytes()).unwrap();

    // // Render the bits into an image.
    // let image = code.render::<Luma<u8>>().build();
    //
    // // Save the image.
    // image.save("/Users/peter/Desktop/qrcode.png").unwrap();

    let image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    println!("{}", image);
}
