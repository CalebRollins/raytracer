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

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\r\n{} {}\r\n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();

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

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical),
            );

            let color = color(&r, Box::new(&world));
            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;
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

#[derive(Copy, Clone)]
struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

impl HitRecord {
    fn zero() -> Self {
        HitRecord {
            t: 0.0,
            p: Vec3::zero(),
            normal: Vec3::zero(),
        }
    }
}

trait Hittable {
	// TODO: Use a Result instead of bool to indicate success
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let mut hit_record = None;
        let mut closest_so_far = t_max;

        for i in 0..self.list.len() {
            if let Some(temp_rec) = self.list[i].hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_record = Some(temp_rec);
            }
		}
		
		hit_record
    }
}

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let mut hit_record = HitRecord::zero();
        let oc = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.point_at_parameter(hit_record.t);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                return Some(hit_record);
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.point_at_parameter(hit_record.t);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                return Some(hit_record);
            }
		}

		None
    }
}
