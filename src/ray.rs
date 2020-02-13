use super::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
	pub fn new(origin: &Vec3, direction: &Vec3) -> Self {
		Ray { origin: origin.clone(), direction: direction.clone() }
	}

    fn point_at_parameter(&self, t: f32) -> Vec3 {
        // TODO: Is this idiomatic?
        self.origin.clone() + self.direction.clone() * t
    }
}
