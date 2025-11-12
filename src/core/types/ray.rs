use crate::core::objects::Hittable;
use crate::scene::Scene;

use super::vector::ToVec3;
use super::{Color, Interval, Point, Vec3};

/// A representation of a ray.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
	/// The starting point of a ray.
	pub origin: Point,
	/// The direction where the ray is pointing.
	pub direction: Vec3,
	/// A measure of how much luminance this ray keeps.
	pub attenuation: Color,
}

// Constructors
impl Ray {
	/// Creates a ray with full attenuation (factor of 1).
	pub fn new(origin: Point, direction: Vec3) -> Self {
		Ray {
			origin,
			direction,
			attenuation: Color::new(1, 1, 1),
		}
	}
	/// Creates a ray with a specified color/attenuation.
	pub fn newc(origin: Point, direction: Vec3, color: Color) -> Self {
		Ray {
			origin,
			direction,
			attenuation: color,
		}
	}
}

// Operations
impl Ray {
	/// Calculates the point where a ray is located at a specific parameter `t`.
	/// This parameter can be (albeit misguidedly) understood as 'time'.
	pub fn at(&self, t: f64) -> Point {
		let point = self.origin.to_vec3() + self.direction.scale(t);
		point.into()
	}
	/// Calculates the color of a ray in the specified scene.
	pub fn color(self, scene: &Scene, bounces: u32) -> Color {
		if bounces == 0 {
			return Color::black();
		}
		// find intersection with an object
		let Some(hit) = scene.hit(self, Interval::from(0.001)) else {
			// white background
			return Color::new(1.0, 1.0, 1.0);
		};
		// determine color recursively
		if let Some(scattered_ray) = hit.material.scatter(self, hit) {
			// ray was scattered
			let color = scattered_ray.color(scene, bounces - 1);
			(scattered_ray.attenuation.to_vec3() * color.to_vec3()).into()
		} else {
			// ray was absorbed
			Color::black()
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::objects::{Material, Sphere};
	use crate::scene::Scene;
	use crate::types::{Color, Point, ToVec3, Vec3};

	use super::Ray;

	#[test]
	fn ray_color_recursion_stops() {
		// This scene has two spheres:
		let sphere1 = Sphere::new(
			Point::new(-1, 0, 0),
			0.5,
			Material::Metal {
				color: Color(1.0, 0.0, 0.0),
				fuzz: 0.0,
			},
		);
		let sphere2 = Sphere::new(
			Point::new(1, 0, 0),
			0.5,
			Material::Metal {
				color: Color(1.0, 0.0, 0.0),
				fuzz: 0.0,
			},
		);
		let scene = Scene::from([sphere1, sphere2]);

		// This ray shoots out from between the spheres towards the center of the left one,
		// and bounces off towards the center of the right one; this continues indefinitely:
		let ray = Ray::new(Point::origin(), Vec3::new(-1, 0, 0));

		// The recursion should stop after 10 bounces:
		let _ = ray.color(&scene, 10);
		// If recursion doesn't stop, stack will overflow
	}

	#[test]
	fn if_empty_scene_then_nonblack_color() {
		// This scene has no objects:
		let scene = Scene::new();
		// This ray shoots out from origin into the view direction:
		let ray = Ray::new(Point::origin(), Vec3::new(0, 0, -1));

		// TODO: adjust when scene supports custom background
		// We should expect the background color:
		let color = ray.color(&scene, 5);
		assert_ne!(
			color,
			Color::black(),
			"color should be the one of the background, but got black"
		)
	}

	#[test]
	fn if_scene_with_objects_then_nonblack_color() {
		// This scene has a red sphere:
		let sphere_pos = Point::new(0, 0, -1);
		let sphere = Sphere::new(
			sphere_pos,
			0.5,
			Material::Matte {
				color: Color(1.0, 0.0, 0.0),
			},
		);
		let scene = Scene::from([sphere]);
		// This ray shoots out from camera center into the sphere:
		let camera_pos = Point::origin();
		let ray = Ray::new(camera_pos, sphere_pos.to_vec3() - camera_pos.to_vec3());

		// TODO: adjust when scene supports custom background (=> non-bg and non-black)
		// We should expect a reddish color:
		let color = ray.color(&scene, 5);
		assert!(
			color.r() > 0.1,
			"color should be reddish, but red channel was below 0.1"
		);
		assert_ne!(
			color,
			Color::black(),
			"color should be the one of the sphere, but got black"
		);
	}

	#[test]
	fn if_ray_absorbed_then_black_color() {
		// This scene has a sphere of absorbant material:
		let sphere_pos = Point::new(0, 0, -1);
		let sphere = Sphere::new(sphere_pos, 0.5, Material::Absorbant);
		let scene = Scene::from([sphere]);
		// This ray shoots out from camera center into the sphere:
		let camera_pos = Point::origin();
		let ray = Ray::new(camera_pos, sphere_pos.to_vec3() - camera_pos.to_vec3());

		// We should expect a black color in just one hit:
		let color = ray.color(&scene, 1);
		assert_eq!(
			color,
			Color::black(),
			"absorbed ray should be black, but was {:?}",
			color
		)
	}
}
