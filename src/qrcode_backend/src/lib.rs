use candid::{CandidType, Deserialize};
use std::{include_bytes, option};

mod core;

const IMAGE_SIZE_IN_PIXELS: usize = 1024;
const LOGO_TRANSPARENCY: &[u8] = include_bytes!("../assets/logo_transparent.png");
const LOGO_WHITE: &[u8] = include_bytes!("../assets/logo_white.png");

#[derive(CandidType, Deserialize)]
struct Options {
    add_logo: bool,
    add_gradient: bool,
    add_transparency: Option<bool>,
}

#[derive(CandidType, Deserialize)]
struct QrError {
    message: String,
}

#[derive(CandidType, Deserialize)]
enum QrResult {
    Image(Vec<u8>),
    Error(QrError),
}

fn qrcode_impl(input: String, options: Options) -> QrResult {
    let logo = if options.add_transparency == Some(true) {
        LOGO_TRANSPARENCY
    } else {
        LOGO_WHITE
    };

    let result = match core::generate(input, options, logo, IMAGE_SIZE_IN_PIXELS) {
        Ok(blob) => QrResult::Image(blob),
        Err(e) => QrResult::Error(QrError {
            message: e.to_string(),
        }),
    };
    result
}

#[ic_cdk::update]
fn qrcode_query(input: String, options: Options) -> QrResult {
    qrcode_impl(input, options)
}

#[ic_cdk::query]
fn qrcode_query(input: String, options: Options) -> QrResult {
    qrcode_impl(input, options)
}