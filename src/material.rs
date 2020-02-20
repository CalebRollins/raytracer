use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Hit>;
}

// impl Clone for Box<dyn Material> {
// 	fn clone(&self) -> Self {

// 	}
// }

impl dyn Material {
    pub fn new() -> Box<dyn Material> {
        Box::new(Lambertian {
            albedo: Vec3::new(0.8, 0.6, 0.2),
        })
    }
}

// TODO: Return this instead of tuple
pub struct Hit {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Hit> {
        let reflected = reflect(&Vec3::unit_vector(&r_in.direction), &rec.normal);
        let scattered = Ray::new(&rec.p, &reflected);
        let attenuation = self.albedo;

        if Vec3::dot(&scattered.direction, &rec.normal) > 0.0 {
            Some(Hit {
                scattered,
                attenuation,
            })
        } else {
            None
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * *n
}
