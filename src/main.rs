mod vec3;
use vec3::Vec3;
mod hittable;
mod ray;
use hittable::{Hittable, HittableList};
mod material;
mod camera;
use camera::Camera;
use rand::prelude::*;

fn main() {
    let nx = 480 * 2;
    let ny = 270 * 2;
    let ns = 100;
	println!("P3\r\n{} {}\r\n255", nx, ny);

	let world = HittableList::random_scene();

    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    let cam = Camera::new(
        look_from,
        look_at,
        up,
        20.0,
        nx as f32 / ny as f32,
        aperature,
        dist_to_focus,
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
