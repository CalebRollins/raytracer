use super::ray::Ray;
use super::vec3::Vec3;
use crate::ray;

#[derive(Copy, Clone)]
pub struct HitRecord {
    t: f32,
    p: Vec3,
    pub normal: Vec3,
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

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
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

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
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
