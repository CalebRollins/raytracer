use super::vec3::Vec3;
use super::HitRecord;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
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
                return true;
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.point_at_parameter(hit_record.t);
                hit_record.normal = (hit_record.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}