use serde::{Deserialize, Serialize};
use serde_repr::*;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum ColorSpace {
    YCbCr,
    RGB,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum ChromaSubsampling {
  Cs420,
  Cs422,
  Cs444,
  Cs400,
}

#[wasm_bindgen(typescript_custom_section)]
const CUSTOM_OPTIONS: &'static str = r#"
interface CustomOptions {
    width: number;
    height: number;
    quality?: number;
    alpha_quality?: number;
    speed?: number;
    premultiplied_alpha?: boolean;
    color_space?: ColorSpace;
    chroma?: ChromaSubsampling;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "CustomOptions")]
    pub type IOptions;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub width: usize,

    pub height: usize,

    pub quality: Option<u8>,

    pub alpha_quality: Option<u8>,

    pub speed: Option<u8>,

    pub premultiplied_alpha: Option<bool>,

    pub color_space: Option<ColorSpace>,

    pub chroma: Option<ChromaSubsampling>
}

impl Options {
    pub fn new(options: IOptions) -> Options {
        let options: JsValue = JsValue::from(options);
        let options: Options = from_value(options).expect("Failed to parse supplied options object!");

        return Options {
            width: options.width,
            height: options.height,
            quality: match options.quality {
              None => Some(75),
              _ => options.quality,
            },
            alpha_quality: match options.alpha_quality{
              None => Some(25),
              _ => options.alpha_quality,
            },
            speed: match options.speed {
              None => Some(6),
              _ => options.speed,
            },
            premultiplied_alpha: match options.premultiplied_alpha {
              None => Some(false),
              _ => options.premultiplied_alpha,
            },
            color_space: match options.color_space {
              None => Some(ColorSpace::YCbCr),
              _ => options.color_space,
            },
            chroma: match options.chroma {
              None => Some(ChromaSubsampling::Cs444),
              _ => options.chroma,
            }
        };
    }
}