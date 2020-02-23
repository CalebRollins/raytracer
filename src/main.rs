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
    let nx = 400;
    let ny = 200;
    let ns = 100;
    println!("P3\r\n{} {}\r\n255", nx, ny);

    let one = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Lambertian::new(0.1, 0.2, 0.5),
    };

    let two = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Lambertian::new(0.8, 0.8, 0.0),
    };

    let three = Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Metal::new(0.8, 0.6, 0.2, 0.3),
    };

    let four = Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Dielectric::new(1.5),
    };

    let five = Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: Dielectric::new(1.5),
    };

    let world = HittableList {
        list: vec![
            Box::new(one),
            Box::new(two),
            Box::new(three),
            Box::new(four),
            Box::new(five),
        ],
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
