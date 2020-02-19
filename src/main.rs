// #![warn(
//     nonstandard_style,
//     warnings,
//     rust_2018_idioms,
//     future_incompatible,
//     clippy::all,
//     clippy::restriction,
//     clippy::pedantic,
//     clippy::cargo
// )]
// #![allow(clippy::integer_arithmetic)]
// #![allow(clippy::missing_inline_in_public_items)]
// #![allow(clippy::multiple_crate_versions)]
// #![allow(clippy::implicit_return)]

mod vec3;
use vec3::Vec3;
mod ray;
use ray::Ray;
mod hittable;
use hittable::{Hittable, HittableList, Sphere};
mod camera;
use camera::Camera;
use rand::prelude::*;

fn main() {
    let nx = 200;
	let ny = 100;
	let ns = 100;
    println!("P3\r\n{} {}\r\n255", nx, ny);

    let x = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let y = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };

    let world = HittableList {
        list: vec![Box::new(x), Box::new(y)],
    };

	let cam = Camera::new();
	let mut rng = thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
			let mut col = Vec3::zero();
			for _ in 0..ns {
				let u = (i as f32 + rng.gen_range(0.0, 1.0)) / nx as f32;
				let v = (j as f32 + rng.gen_range(0.0, 1.0)) / ny as f32;
				let r = cam.get_ray(u, v);
				col += &color(&r, Box::new(&world));
			}

            col /= ns as f32;
            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: &ray::Ray, world: Box<&dyn Hittable>) -> Vec3 {
    return if let Some(temp_rec) = world.hit(r, 0.0, std::f32::MAX) {
        0.5 * Vec3::new(
            temp_rec.normal.x() + 1.0,
            temp_rec.normal.y() + 1.0,
            temp_rec.normal.z() + 1.0,
        )
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
    };
}
