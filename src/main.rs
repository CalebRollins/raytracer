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
    for j in (0..ny).rev() {
        for i in 0..nx {
			let u = i as f32 / nx as f32;
			let v = j as f32 / ny as f32;

			let r = Ray::new(&origin, &(lower_left_corner.clone() + u * horizontal.clone() + v * vertical.clone()));
            let color = color(&r);
            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
	let oc = r.origin.clone() - center.clone();
	let a = Vec3::dot(&r.direction, &r.direction);
	let b = 2.0 * Vec3::dot(&oc, &r.direction);
	let c = Vec3::dot(&oc, &oc) - radius * radius;
	let discriminant = b*b - 4.0*a*c;
	return discriminant > 0.0;
}

fn color(r: &ray::Ray) -> Vec3 {
	if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &r) {
		return Vec3::new(1.0, 0.0, 0.0);
	}
    let unit_direction = Vec3::unit_vector(&r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
}