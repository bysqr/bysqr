use std::fs;
use std::path::PathBuf;
use base64::Engine;

mod models;
mod encoder;
use qrcode::QrCode;
// use image::Luma;
use qrcode::render::unicode;

use crate::models::Pay;
use clap::{Parser, Subcommand};
use image::{ImageEncoder, Luma};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        #[arg(long = "xml", required = false)]
        xml: Option<PathBuf>,

        #[arg(long = "show", required = false)]
        show: bool,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Encode { xml, show }) => {
            if let Some(path) = xml {
                let xml_content = fs::read_to_string(&path).expect("unable to read XML file");
                let pay: Pay = quick_xml::de::from_str(&xml_content).expect("unable to decode XML file");
                let encoded = encoder::encode(&pay);
                let code = QrCode::new(encoded.as_bytes()).expect("unable to create QR code");

                if *show {
                    let image = code.render::<unicode::Dense1x2>()
                        .dark_color(unicode::Dense1x2::Light)
                        .light_color(unicode::Dense1x2::Dark)
                        .build();

                    println!("{}", image);
                } else {
                    let image = code.render::<Luma<u8>>()
                        .min_dimensions(160, 160)
                        .build();

                    let mut buffer = Vec::new();
                    image::codecs::png::PngEncoder::new(&mut buffer).write_image(
                        &image,
                        image.width(),
                        image.height(),
                        image::ExtendedColorType::L8,
                    ).expect("unable to write image");

                    let base64_content = base64::prelude::BASE64_STANDARD.encode(&buffer);
                    let base64_image = format!("data:image/png;base64,{}", base64_content);
                    println!("{}", base64_image);
                }
            } else {
                panic!("no XML path defined to encode")
            }
        }
        None => {}
    }

    // let xml = fs::read_to_string("/Users/peter/code/rust/bysqr/test.xml").unwrap();
    //
    // let pay: Pay = quick_xml::de::from_str(&xml).unwrap();
    //
    // let encoded = encoder::encode(&pay);
    //
    // let code = QrCode::new(encoded.as_bytes()).unwrap();

    // // Render the bits into an image.
    // let image = code.render::<Luma<u8>>().build();
    //
    // // Save the image.
    // image.save("/Users/peter/Desktop/qrcode.png").unwrap();

    // let image = code.render::<unicode::Dense1x2>()
    //     .dark_color(unicode::Dense1x2::Light)
    //     .light_color(unicode::Dense1x2::Dark)
    //     .build();
    //
    // println!("{}", image);
}
