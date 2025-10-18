use std::io::{self, BufWriter, Write};

use crate::core::output::calc_colors;
use crate::core::types::Image;

/// Outputs the image to the specified `writer` in plain (ASCII) format.
pub fn plain<W: Write>(image: &Image, gamma: f64, writer: &mut W) -> Result<(), io::Error> {
	let mut writer = BufWriter::new(writer);
	writeln!(writer, "P3\n{} {}\n255", image.width(), image.height())?;
	for line in image {
		for pixel in line {
			let (r, g, b) = calc_colors(pixel, gamma);
			writeln!(writer, "{} {} {}", r, g, b)?;
		}
	}
	writer.flush()?;
	Ok(())
}

/// Outputs the image to the specified `writer` in raw (binary) format.
pub fn raw<W: Write>(image: &Image, gamma: f64, writer: &mut W) -> Result<(), io::Error> {
	let mut writer = BufWriter::new(writer);
	writeln!(writer, "P6\n{} {}\n255", image.width(), image.height())?;
	for line in image {
		for pixel in line {
			let (r, g, b) = calc_colors(pixel, gamma);
			writer.write_all(&[r, g, b])?;
		}
	}
	writer.flush()?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use crate::core::types::{Color, Image};

	use super::calc_colors;

	#[test]
	fn transforms_color_to_output_rgb() {
		// This pixel color is represented in internal, linear coordinate system:
		let pixel = Color::new(0, 1, 0.5);
		// This gamma value should be used for gamma correction:
		let gamma = 2.4;

		// In output space, the color should be a gamma-corrected 8-bit value:
		let actual: (u8, u8, u8) = calc_colors(&pixel, gamma);
		let expected: (u8, u8, u8) = (0, 255, (pixel.b().powf(1.0 / gamma) * 256.0) as u8);
		assert_eq!(
			expected, actual,
			"output color should be {:?}, but was {:?}",
			expected, actual,
		);
	}

	#[test]
	fn correct_plain_ppm() {
		// This is a 2x2 image:
		let mut image = Image::init(2, 2);
		// The bottom right pixel is red:
		image[(1, 1)] = Color::new(1, 0, 0);

		// The output should match the plain .ppm format:
		let expected = "P3
2 2
255
0 0 0
0 0 0
0 0 0
255 0 0
";

		// Write image to buf:
		let mut buf: Vec<u8> = Vec::new();
		let write_result = super::plain(&image, 2.2, &mut buf);
		assert!(write_result.is_ok(), "writing should succeed, but didn't");

		let decode_result = String::from_utf8(buf);
		assert!(
			decode_result.is_ok(),
			"converting from utf-8 should succeed, but didn't"
		);

		let actual = decode_result.unwrap();
		assert_eq!(actual, expected, ".ppm output should match, but didn't");
	}

	#[test]
	fn correct_raw_ppm() {
		// This is a 2x2 image:
		let mut image = Image::init(2, 2);
		// The bottom right pixel is red:
		image[(1, 1)] = Color::new(1, 0, 0);

		// The output should match the raw .ppm format:
		#[rustfmt::skip]
		let expected = {
			let mut bytes = Vec::new();
			bytes.append(&mut "P6\n2 2\n255\n".as_bytes().to_vec());
			bytes.append(&mut [
				0,   0, 0,
				0,   0, 0,
				0,   0, 0,
				255, 0, 0,
			].to_vec());
			bytes
		};

		// Write image to buf:
		let mut buf: Vec<u8> = Vec::new();
		let write_result = super::raw(&image, 2.2, &mut buf);
		assert!(write_result.is_ok(), "writing should succeed, but didn't");
		assert_eq!(expected, buf, ".ppm output should match, but didn't");
	}
}
