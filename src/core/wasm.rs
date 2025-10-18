use wasm_bindgen::prelude::*;

use crate::camera::{Camera, CameraSetup};
use crate::core::output;
use crate::input::RaytracerInput;
use crate::scene::Scene;
use crate::types::ToVec3;

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

#[wasm_bindgen]
pub fn render(input: JsValue) -> Vec<u8> {
	let Ok(input) = RaytracerInput::try_from(input) else {
		return vec![];
	};

	let default_focus_distance =
		(input.camera.source.to_vec3() - input.camera.target.to_vec3()).norm();
	let setup = CameraSetup {
		width: 600,
		height: 300,
		v_fov: 27.0,        //input.camera.fov,
		defocus_angle: 0.0, // input.camera.aperture,
		focus_distance: default_focus_distance,
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
	let Ok(()) = output::px::rgba(&image, 2.2, &mut buf) else {
		return vec![];
	};

	buf
}
