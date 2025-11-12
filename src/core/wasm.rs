use wasm_bindgen::prelude::*;

use crate::camera::{Camera, CameraSetup};
use crate::core::output;
use crate::input::RaytracerInput;
use crate::scene::Scene;

const PREFIX: &str = "raytracer-rust";

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn console_log(s: &str);
}

macro_rules! console_log {
	($($t:tt)*) => {
		console_log(&format!($($t)*));
	};
}

macro_rules! error {
	($($t:tt)*) => {
		{
			let message = format!($($t)*);
			format!("{PREFIX}: {message}")
		}
	};
}

#[wasm_bindgen]
pub fn render(input: JsValue, width: usize, height: usize) -> Result<Vec<u8>, String> {
	let input = RaytracerInput::try_from(input).map_err(|e| error!("invalid input: {e}"))?;

	let setup = CameraSetup {
		width,
		height,
		v_fov: input.camera.fov,
		defocus_angle: input.camera.aperture,
		focus_distance: input.camera.focus_distance,
		lookfrom: input.camera.source,
		lookat: input.camera.target,
		..Default::default()
	};

	let camera = Camera::from(setup).anti_aliasing(10).bounces(10);
	let scene = Scene::from_objs(input.scene);

	console_log!("raytracer: rendering...");
	let image = camera.render(&scene);
	console_log!("raytracer: rendering finished.");

	let mut buf = Vec::with_capacity(4 * image.width() * image.height());
	output::px::rgba(&image, 2.2, &mut buf)
		.map_err(|e| error!("could not write image pixels: {e}"))?;

	Ok(buf)
}
