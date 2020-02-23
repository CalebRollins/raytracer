use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;
use dyn_clone::DynClone;

pub trait Material: DynClone {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Hit>;
}

// TODO: Return this instead of tuple
pub struct Hit {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(x: f32, y: f32, z: f32) -> Box<dyn Material> {
        Box::new(Lambertian {
            albedo: Vec3::new(x, y, z),
        })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<Hit> {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        let scattered = Ray::new(&rec.p, &target);
        let attenuation = self.albedo;

        Some(Hit {
            scattered,
            attenuation,
        })
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(x: f32, y: f32, z: f32, fuzz: f32) -> Box<dyn Material> {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Box::new(Metal {
            albedo: Vec3::new(x, y, z),
            fuzz,
        })
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Hit> {
        let reflected = reflect(&Vec3::unit_vector(&r_in.direction), &rec.normal);
        let scattered = Ray::new(
            &rec.p,
            &(reflected + self.fuzz * Vec3::random_in_unit_sphere()),
        );
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
