mod alpha;
mod encode;
mod options;
mod utils;

use rgb::{RGBA, FromSlice};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn encode(pixels: &[u8], options: options::IOptions) -> Vec<u8> {
    utils::set_panic_hook();
    let options = options::Options::new(options);
    let width = options.width;
    let height = options.height;
    let raw_pixels = pixels.to_vec();

    let pixels: Vec<RGBA<u8>> = match (raw_pixels.len() / (width * height)) % 4 {
        0 => pixels.as_rgba().to_vec(),
        _ => alpha::fill_alpha(pixels.as_rgb()),
    };

    return encode::encode_to_avif(&pixels, &options).expect("Failed to encode AVIF! image");
}
