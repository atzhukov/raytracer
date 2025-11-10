use std::io::{self, BufWriter, Write};

use crate::core::output::calc_colors;
use crate::core::types::Image;

/// Outputs the image to the specified `writer` in raw (binary) format, containing only
/// the pixels' color & alpha channel values (RGBA) and no other information.
pub fn rgba<W: Write>(image: &Image, gamma: f64, writer: &mut W) -> Result<(), io::Error> {
	let mut writer = BufWriter::new(writer);
	for line in image {
		for pixel in line {
			let (r, g, b) = calc_colors(pixel, gamma);
			writer.write_all(&[r, g, b, 255])?;
		}
	}
	writer.flush()?;
	Ok(())
}
