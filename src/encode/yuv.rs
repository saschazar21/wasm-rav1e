use rgb::RGBA;

#[cfg(target_feature = "simd128")]
use std::simd::{ f32x4 };

fn clamp(val: f32) -> u8 {
  return (val.round() as u8).max(0_u8).min(255_u8);
}

#[cfg(target_feature = "simd128")]
unsafe fn sum_f32x4(val: f32x4) -> f32 {
  return val.0 + val.1 + val.2 + val.3;
}

#[cfg(target_feature = "simd128")]
unsafe fn to_ycbcr(pixel: &RGBA<u8>) -> (u8, u8, u8, u8) {
  let rgb: f32x4 = f32x4(pixel.r as f32, pixel.g as f32, pixel.b as f32, 1.0);
  let y  = sum_f32x4(rgb * f32x4( 0.2567882353, 0.5041294118, 0.09790588235, 0.06274509804));
  let cb = sum_f32x4(rgb * f32x4(-0.1482235294, -0.2909921569, 0.4392156863, 0.5));
  let cr = sum_f32x4(rgb * f32x4( 0.4392156863, -0.3677882353, -0.07142745098, 0.5));

  return (clamp(y), clamp(cb), clamp(cr), pixel.a);
}

#[cfg(not(target_feature = "simd128"))]
fn to_ycbcr(pixel: &RGBA<u8>) -> (u8, u8, u8, u8) {
  let r = pixel.r as f32;
  let g = pixel.g as f32;
  let b = pixel.b as f32;

  let y = 16_f32 + (65.481 * r + 128.553 * g + 24.966 * b) / 255_f32;
  let cb = 128_f32 + (-37.797 * r - 74.203 * g + 112.000 * b) / 255_f32;
  let cr = 128_f32 + (112.000 * r - 93.786 * g - 18.214 * b) / 255_f32;

  return (clamp(y), clamp(cb), clamp(cr), pixel.a);
}

pub fn rgb_to_ycbcr(pixels: &[RGBA<u8>]) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
  let mut y_plane = Vec::new();
  let mut cb_plane = Vec::new();
  let mut cr_plane = Vec::new();
  let mut a_plane = Vec::new();

  pixels
    .to_vec()
    .iter()
    .map(to_ycbcr)
    .for_each(|(y, cb, cr, a)| {
      y_plane.push(y);
      cb_plane.push(cb);
      cr_plane.push(cr);
      a_plane.push(a);
    });

  return (y_plane, cb_plane, cr_plane, a_plane);
}