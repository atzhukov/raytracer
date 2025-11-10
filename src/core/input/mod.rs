use std::fs;
use std::io::BufReader;

use super::objects::Object;
use super::types::Point;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A type that represents input to the raytracer.
pub struct RaytracerInput {
	/// Camera settings.
	pub camera: CameraInput,
	/// Objects in the scene.
	pub scene: Vec<Object>,
}
impl TryFrom<&str> for RaytracerInput {
	type Error = String;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		serde_json::from_str::<Self>(value).map_err(|e| e.to_string())
	}
}
impl TryFrom<fs::File> for RaytracerInput {
	type Error = String;
	fn try_from(value: fs::File) -> Result<Self, Self::Error> {
		let reader = BufReader::new(value);
		serde_json::from_reader(reader).map_err(|e| e.to_string())
	}
}
#[cfg(target_arch = "wasm32")]
impl TryFrom<wasm_bindgen::JsValue> for RaytracerInput {
	type Error = String;
	fn try_from(value: wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
		serde_wasm_bindgen::from_value::<Self>(value).map_err(|e| e.to_string())
	}
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A type that represents a subset of camera settings settable via input.
pub struct CameraInput {
	/// The vertical field of view, in degrees.
	pub fov: f64,
	/// The position of the camera.
	pub source: Point,
	/// The point the camera is looking at.
	pub target: Point,
	/// Angular aperture size, in degrees.
	pub aperture: f64,
	/// Distance from camera center to the plane where the objects are in focus.
	pub focus_distance: f64,
}

#[cfg(test)]
mod tests {

	use crate::core::input::CameraInput;
	use crate::objects::{Material, Sphere, ToObject};
	use crate::types::{Color, Point};

	use super::RaytracerInput;

	#[test]
	fn if_input_valid_then_parsed_value_should_have_correct_fields() {
		// This is the input string:
		let input = r#"{
			"camera": {
				"fov": 27.0,
				"source": [0.0, 0.0, -1.0],
				"target": [0.0, 0.0, 0.0],
				"aperture": 0.0,
				"focusDistance": 0.0
			},
			"scene": [
				{
					"type": "sphere",
					"center": [0.0, 0.0, 0.0],
					"radius": 1.5,
					"material": {
						"type": "metal",
						"color": [0.5, 0.2, 0.1],
						"fuzz": 0.5
					}
				}
			]
		}"#;
		// This is the value the input should be parsed into:
		let expected = RaytracerInput {
			camera: CameraInput {
				fov: 27.0,
				source: Point::new(0, 0, -1),
				target: Point::origin(),
				aperture: 0.0,
				focus_distance: 0.0,
			},
			scene: vec![
				Sphere::new(
					Point::origin(),
					1.5,
					Material::Metal {
						color: Color::new(0.5, 0.2, 0.1),
						fuzz: 0.5,
					},
				)
				.wrap(),
			],
		};

		// Parsing should not result in an error, and the values should match:
		let result = RaytracerInput::try_from(input);
		assert!(
			result.is_ok(),
			"input should be parsed, but error occurred: {:?}",
			result.err()
		);

		let parsed = result.unwrap();
		assert_eq!(
			expected, parsed,
			"parsed input did not match expected value"
		);
	}

	#[test]
	fn if_fields_missing_then_parsing_should_error() {
		// This is the input string, missing a 'source' field:
		let input = r#"{
			"camera": {
				"fov": 27.0,
				"target": [0.0, 0.0, 0.0],
				"aperture": 0.0,
				"focusDistance": 0.0
			},
			"scene": []
		}"#;

		let parsed = RaytracerInput::try_from(input);
		assert!(parsed.is_err(), "parsing should fail, but was successful")
	}
}
