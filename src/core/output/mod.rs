use crate::types::{Color, ToVec3};

pub mod px;
pub mod ppm;

/// Performs gamma correction and translation from internal to output color space.
/// Returns a tuple `(red, green, blue)` with each value corresponding to the respective channel's
/// 8-bit value.
fn calc_colors(pixel: &Color, gamma: f64) -> (u8, u8, u8) {
	let rgb = pixel.to_vec3().exp(1.0 / gamma);
	rgb.to_tuple(|x| (256.0 * x.clamp(0.0, 0.999)) as u8)
}
