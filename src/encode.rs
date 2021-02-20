mod yuv;

use crate::options::{ ChromaSubsampling, ColorSpace, Options };

use avif_serialize::Aviffy;
use rav1e::prelude::*;
use rav1e::color::ChromaSampling;
use rgb::{ RGBA };

const BIT_DEPTH: usize = 8;
const NUM_CPUS: usize = 1;

fn split_rgb_planes(pixels: &[RGBA<u8>]) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
  let mut r_plane = Vec::new();
  let mut g_plane = Vec::new();
  let mut b_plane = Vec::new();
  let mut a_plane = Vec::new();

  pixels
    .to_vec()
    .iter()
    .for_each(|pixel| {
      r_plane.push(pixel.r);
      g_plane.push(pixel.g);
      b_plane.push(pixel.b);
      a_plane.push(pixel.a);
    });

  return (g_plane, b_plane, r_plane, a_plane);
}

fn quality_to_quantizer(val: usize) -> u8 {
  return ((1_f32 - (val as f32) * 0.01) * 255_f32).round().max(0_f32).min(255_f32) as u8;
}

fn encode_to_av1(planes: &[&[u8]], options: &Options, quality: usize, pixel_range: PixelRange, chroma_sampling: ChromaSampling, color_description: Option<ColorDescription>) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {  
  let encoder_config = EncoderConfig {
    width: options.width,
    height: options.height,
    time_base: Rational::new(1, 1),
    sample_aspect_ratio: Rational::new(1, 1),
    bit_depth: BIT_DEPTH,
    chroma_sampling,
    chroma_sample_position: match chroma_sampling {
      ChromaSampling::Cs400 => ChromaSamplePosition::Unknown,
      _ => ChromaSamplePosition::Colocated,
    },
    pixel_range,
    color_description,
    mastering_display: None,
    content_light: None,
    enable_timing_info: false,
    still_picture: true,
    error_resilient: false,
    switch_frame_interval: 0,
    min_key_frame_interval: 0,
    max_key_frame_interval: 0,
    reservoir_frame_delay: None,
    low_latency: false,
    quantizer: quality_to_quantizer(quality) as usize,
    min_quantizer: quality_to_quantizer(quality),
    bitrate: 0,
    tune: Tune::Psychovisual,
    tile_cols: 0,
    tile_rows: 0,
    tiles: NUM_CPUS,
    rdo_lookahead_frames: 1,
    speed_settings: SpeedSettings::from_preset(match options.speed {
      Some(s) => s as usize,
      _ => 6,
    }),
  };

  let config = Config::new()
    .with_threads(NUM_CPUS)
    .with_encoder_config(encoder_config);

  let mut ctx: Context<u8> = config.new_context()?;
  let mut frame = ctx.new_frame();

  for (dst, src) in frame.planes.iter_mut().zip(planes) {
    dst.copy_from_raw_u8(src, options.width, 1);
  }

  ctx.send_frame(frame)?;
  ctx.flush();

  let mut result = Vec::new();

  loop {
    match ctx.receive_packet() {
      Ok(mut packet) => match packet.frame_type {
        FrameType::KEY => result.append(&mut packet.data),
        _ => continue,
      },
      Err(EncoderStatus::Encoded) => (),
      Err(EncoderStatus::LimitReached) => break,
      Err(err) => Err(err)?,
    }
  }

  return Ok(result);
}

pub fn encode_to_avif(pixels: &[RGBA<u8>], options: &Options) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
  let quality = match options.quality {
    Some(q) => q,
    _ => 75,
  };

  let alpha_quality = match options.alpha_quality {
    Some(q) => q,
    _ => 75,
  };
  
  let (pixel_range, matrix_coefficients) = match options.color_space {
    Some(ColorSpace::YCbCr) => (PixelRange::Limited, MatrixCoefficients::BT709),
    _ => (PixelRange::Full, MatrixCoefficients::Identity),
  };
  
  let color_description = Some(ColorDescription {
    transfer_characteristics: TransferCharacteristics::SRGB,
    color_primaries: ColorPrimaries::BT601,
    matrix_coefficients,
  });

  let chroma_sampling: ChromaSampling = match options.chroma {
    Some(ChromaSubsampling::Cs444) => ChromaSampling::Cs444,
    Some(ChromaSubsampling::Cs422) => ChromaSampling::Cs422,
    Some(ChromaSubsampling::Cs420) => ChromaSampling::Cs420,
    Some(ChromaSubsampling::Cs400) => ChromaSampling::Cs400,
    _ => ChromaSampling::Cs444,
  };
  
  let (y, u, v, a) = match options.color_space {
    Some(ColorSpace::YCbCr) => yuv::rgb_to_ycbcr(pixels),
    _ => split_rgb_planes(pixels),
  };

  let use_alpha: bool = a.iter().copied().any(|val| val != 255);

  let planes: Vec<&[u8]> = vec![&y, &u, &v];
  let alpha_plane: Vec<&[u8]> = vec![&a];

  let chroma = encode_to_av1(&planes, options, quality as usize, pixel_range, chroma_sampling, color_description);
  let alpha = match use_alpha {
    true => Some(encode_to_av1(&alpha_plane, options, alpha_quality as usize, PixelRange::Full, ChromaSampling::Cs400, None)),
    false => None,
  };

  let (chroma, alpha) = (chroma?, alpha.transpose()?);

  let result = Aviffy::new()
    .premultiplied_alpha(match options.premultiplied_alpha {
      Some(x) => x,
      _ => false,
    })
    .to_vec(&chroma, alpha.as_deref(), options.width as u32, options.height as u32, BIT_DEPTH as u8);

  return Ok(result);
}