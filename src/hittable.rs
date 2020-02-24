use super::material::{Dielectric, Metal, Lambertian, Material};
use super::ray::Ray;
use super::vec3::Vec3;
use rand::prelude::*;

pub struct HitRecord {
    t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
}

impl HitRecord {
    fn zero() -> Self {
        HitRecord {
            t: 0.0,
            p: Vec3::zero(),
            normal: Vec3::zero(),
            material: Lambertian::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn random_scene() -> Self {
        let mut list: Vec<Box<dyn Hittable>> = Vec::new();
		let zero = Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: Lambertian::new(0.5, 0.5, 0.5), };
		list.push(Box::new(zero));

        for a in -5..5 {
            for b in -5..5 {
                let choose_mat = random::<f32>();
                let center = Vec3::new(
                    a as f32 + 0.9 * random::<f32>(),
                    0.2,
                    b as f32 + 0.9 * random::<f32>(),
                );
                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
						// diffuse
						let sphere = Sphere {
                            center,
                            radius: 0.2,
                            material: Lambertian::new(
								random::<f32>() * random::<f32>(),
								random::<f32>() * random::<f32>(),
								random::<f32>() * random::<f32>() 
							),
                        };
                        list.push(Box::new(sphere));
                    } else if choose_mat < 0.95 {
						// metal
						let sphere = Sphere {
                            center,
                            radius: 0.2,
                            material: Metal::new(
								0.5 * (1.0 + random::<f32>()),
								0.5 * (1.0 + random::<f32>()),
								0.5 * (1.0 + random::<f32>()),
								0.5 * random::<f32>(),
							),
                        };
                        list.push(Box::new(sphere));
					} else {
						// glass
						let sphere = Sphere {
							center,
							radius: 0.2,
							material: Dielectric::new(1.5)
						};
						list.push(Box::new(sphere))
					}
                }
            }
		}
		let one = Sphere { center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0, material: Dielectric::new(1.5) };
		list.push(Box::new(one));
		let two= Sphere { center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0, material: Lambertian::new(0.4, 0.2, 0.1) };
		list.push(Box::new(two));
		let three= Sphere { center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0, material: Metal::new(0.7, 0.6, 0.5, 0.0) };
		list.push(Box::new(three));

        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
                hit_record.material = dyn_clone::clone_box(&*self.material);
                return Some(hit_record);
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.point_at_parameter(hit_record.t);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                hit_record.material = dyn_clone::clone_box(&*self.material);
                return Some(hit_record);
            }
        }

        None
    }
}
