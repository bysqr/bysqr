use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use bysqr::{encoder, qr};
use bysqr::models::{try_deserialize_pay, Pay};
#[path = "../preview.rs"]
#[cfg(feature = "preview")]
mod preview;
#[path = "../utils.rs"]
mod utils;
use utils::ensure_directory_for_file;
use serde_json;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        #[arg(long = "src", required = false)]
        src: Option<String>,

        #[arg(long = "format", required = false)]
        format: Option<String>,

        #[arg(long = "preview", required = false)]
        preview: bool,

        #[arg(long = "size", required = false, default_value = "512")]
        size: u32,

        #[arg(long = "quality", required = false, default_value = "90")]
        quality: u8,

        #[arg(long = "save", required = false)]
        save: Option<PathBuf>,

        #[arg(long = "overwrite", required = false)]
        overwrite: bool,
    }
}

#[derive(Debug)]
enum OutputFormat {
    SVG, PNG, JPEG
}

#[derive(Debug)]
enum OutputMode {
    Save(PathBuf, OutputFormat),
    Print(OutputFormat)
}

fn guess_output_mode(destination: &Option<PathBuf>, requested_format: &Option<String>) -> Result<OutputMode, String> {
    if let Some(dest) = destination {
        if let Some(file_ext) = dest.extension() {
            let format = file_ext.to_str().expect("unable to parse extension");

            if format == "png" {
                Ok(OutputMode::Save(PathBuf::from(dest), OutputFormat::PNG))
            } else if format == "jpg" || format == "jpeg" {
                Ok(OutputMode::Save(PathBuf::from(dest), OutputFormat::JPEG))
            } else if format == "svg" {
                Ok(OutputMode::Save(PathBuf::from(dest), OutputFormat::SVG))
            } else {
                Err(format!("invalid output: extension {} is not supported", format))
            }
        } else {
            Err(String::from("invalid output: unable to guess output file format"))
        }
    } else {
        if let Some(format) = requested_format {
            if format == "png" {
                Ok(OutputMode::Print(OutputFormat::PNG))
            } else if format == "jpg" || format == "jpeg" {
                Ok(OutputMode::Print(OutputFormat::JPEG))
            } else if format == "svg" {
                Ok(OutputMode::Print(OutputFormat::SVG))
            } else {
                Err(format!("invalid output: extension {} is not supported", format))
            }
        } else {
            Err(String::from("missing format: when outputing to standard output, a format option is required"))
        }
    }
}

fn deserialize_pay(source: &str) -> Pay {
    if fs::exists(source).unwrap() {
        let content = fs::read_to_string(&source).expect("unable to read source file");

        try_deserialize_pay(&content)
    } else if source.trim_start().starts_with("<?xml") {
        quick_xml::de::from_str(&source).expect("unable to parse source as XML")
    } else if source.trim_start().starts_with("{") {
        serde_json::from_str(&source).expect("unable to parse source as JSON")
    } else {
        panic!("source does not seems to be a valid file or XML or JSON content")
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        None => {}
        Some(Commands::Encode { src, preview, format, save, size, quality, overwrite }) => {
            if let Some(source) = src {
                let pay: Pay = deserialize_pay(&source);
                let encoded = encoder::encode(&pay);

                let svg_code = qr::create_pay_svg(&encoded, qr::Theme::default());

                if *preview {
                    #[cfg(feature = "preview")]
                    {
                        preview::show_svg(svg_code.clone());
                    }

                    #[cfg(not(feature = "preview"))]
                    {
                        panic!("Unable to run preview. This binary wasnt compiled with preview feature");
                    }
                } else {
                    let output_mode = guess_output_mode(save, format).expect("unable to guess output file format");

                    match output_mode {
                        OutputMode::Save(destination, format) => {
                            if destination.exists() && !*overwrite {
                                panic!("output file already exists");
                            }

                            let content = match format {
                                OutputFormat::SVG => {
                                    svg_code
                                }
                                OutputFormat::PNG => {
                                    qr::render_png(&svg_code, *size)
                                }
                                OutputFormat::JPEG => {
                                    qr::render_jpeg(&svg_code, *size, *quality)
                                }
                            };

                            if destination.exists() {
                                fs::remove_file(&destination).expect("unable to remove existing file");
                            }

                            ensure_directory_for_file(&destination);

                            fs::write(&destination, content).expect("unable to write output file");
                        }
                        OutputMode::Print(format) => {
                            match format {
                                OutputFormat::SVG => {
                                    println!("{}", String::from_utf8(svg_code).expect("unable to decode XML content"));
                                }
                                OutputFormat::PNG => {
                                    println!("{}", qr::to_base64_png(&svg_code, *size));
                                }
                                OutputFormat::JPEG => {
                                    println!("{}", qr::to_base64_jpeg(&svg_code, *size, *quality));
                                }
                            }
                        }
                    }
                }
            } else {
                panic!("unable to read source");
            }
        }
    }
}
