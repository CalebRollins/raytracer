mod vec3;
use vec3::Vec3;
mod hittable;
mod ray;
use hittable::{Hittable, HittableList, Sphere};
mod material;
use material::{Dielectric, Lambertian, Metal};
mod camera;
use camera::Camera;
use rand::prelude::*;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\r\n{} {}\r\n255", nx, ny);
    let r = (std::f32::consts::PI / 4.0).cos();
    let one = Sphere {
        center: Vec3::new(-r, 0.0, -1.0),
        radius: r,
        material: Lambertian::new(0.0, 0.0, 1.0),
    };

    let two = Sphere {
        center: Vec3::new(r, 0.0, -1.0),
        radius: r,
        material: Lambertian::new(1.0, 0.0, 0.0),
    };
    let world = HittableList {
        list: vec![Box::new(one), Box::new(two)],
    };

    let look_from = Vec3::new(3.0, 3.0, 2.0);
	let look_at = Vec3::new(0.0, 0.0, -1.0);
	let dist_to_focus = (look_from - look_at).length();
    let cam = Camera::new(
		look_from,
		look_at,
		Vec3::new(0.0, 1.0, 0.0),
		20.0,
		nx as f32 / ny as f32,
		2.0,
		dist_to_focus
	);

    let mut rng = thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let u = (i as f32 + rng.gen_range(0.0, 1.0)) / nx as f32;
                let v = (j as f32 + rng.gen_range(0.0, 1.0)) / ny as f32;
                let r = cam.get_ray(u, v);
                col += &color(&r, Box::new(&world), 0);
            }

            col /= ns as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: &ray::Ray, world: Box<&dyn Hittable>, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, std::f32::MAX) {
        if let Some(hit) = rec.material.scatter(&r, &rec) {
            if depth < 50 {
                hit.attenuation * color(&hit.scattered, world, depth + 1)
            } else {
                // TODO: Figure out a cleaner way to do this
                // https://github.com/rust-lang/rfcs/issues/2411
                Vec3::zero()
            }
        } else {
            Vec3::zero()
        }
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
