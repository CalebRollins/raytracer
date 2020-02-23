use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;
use dyn_clone::DynClone;
use rand::prelude::*;

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

#[derive(Clone)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Box<dyn Material> {
        Box::new(Dielectric { ref_idx })
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Hit> {
        let outward_normal;
        let ni_over_nt;
        let cosine;

        if Vec3::dot(&r_in.direction, &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine =
                self.ref_idx * Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.length();
        }

        let reflect_prob;
		let mut refracted = Vec3::zero();
		if let Some(refract) = refract(&r_in.direction, &outward_normal, ni_over_nt) {
			refracted = refract;
			reflect_prob = schlick(cosine, self.ref_idx)
		} else {
			reflect_prob = 1.0;
		};

        let mut rng = thread_rng();
        let reflected = reflect(&r_in.direction, &rec.normal);
        let scattered = if rng.gen_range(0.0, 1.0) < reflect_prob {
            Ray::new(&rec.p, &reflected)
        } else {
            Ray::new(&rec.p, &refracted)
        };

        Some(Hit {
            scattered,
            attenuation: Vec3::new(1.0, 1.0, 0.0),
        })
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * *n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    return if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    };
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r1 = r0 * r0;
    r1 + (1.0 - r1) * (1.0 - cosine).powi(5)
}
