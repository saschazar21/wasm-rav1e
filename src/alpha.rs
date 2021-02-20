use rgb::{ RGB, RGBA };

pub fn fill_alpha(pixels: &[RGB<u8>]) -> Vec<RGBA<u8>> {
  return pixels
      .iter()
      .map(|p| p.alpha(255))
      .collect();
}