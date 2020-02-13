use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

// TODO: Consider three separate variables instead of an array
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn zero() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn one() -> Self {
        Vec3 { e: [1.0, 1.0, 1.0] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    fn squared_length(&self) -> f32 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn cross(&self, other: &Self) -> Self {
        Self {
            e: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    // TODO: Will we need equivalents of these?
    // 	inline std::istream& operator>>(std::istream &is, vec3 &t) {
    //     is >> t.e[0] >> t.e[1] >> t.e[2];
    //     return is;
    // }

    // inline std::ostream& operator<<(std::ostream &os, const vec3 &t) {
    //     os << t.e[0] << " " << t.e[1] << " " << t.e[2];
    //     return os;
    // }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Self::new(self.x(), self.y(), self.z()) 
    }
}

// TODO: See if we will actually need the unary positive sign, since there isn't a trait for this.

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl AddAssign<&Self> for Vec3 {
    fn add_assign(&mut self, other: &Self) {
        self[0] += other[0];
        self[1] += other[1];
        self[2] += other[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl SubAssign<&Self> for Vec3 {
    fn sub_assign(&mut self, other: &Self) {
        self[0] -= other[0];
        self[1] -= other[1];
        self[2] -= other[2];
    }
}

impl Mul<Self> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Self) -> Self::Output {
        Vec3::new(self.x() * other.x(), self.y() * other.y(), self.z() * other.z())
    }
}

fn vec_mul_by_f32(v: &Vec3, t: f32) -> Vec3 {
	Vec3::new(v.x() * t, v.y() * t, v.z() * t)
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f32) -> Self::Output {
		vec_mul_by_f32(&self, t)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
		vec_mul_by_f32(&v, self)
    }
}

impl MulAssign<&Self> for Vec3 {
    fn mul_assign(&mut self, other: &Self) {
        self[0] *= other[0];
        self[1] *= other[1];
        self[2] *= other[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self[0] *= t;
        self[1] *= t;
        self[2] *= t;
    }
}

impl Div<Self> for &Vec3 {
    type Output = Vec3;
    fn div(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.x() / other.x(),
                self.y() / other.y(),
                self.z() / other.z(),
            ],
        }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, t: f32) -> Self::Output {
        Vec3 {
            e: [self.x() / t, self.y() / t, self.z() / t],
        }
    }
}

impl DivAssign<&Self> for Vec3 {
    fn div_assign(&mut self, other: &Self) {
        self[0] /= other[0];
        self[1] /= other[1];
        self[2] /= other[2];
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        let k = 1.0 / t;
        self[0] /= k;
        self[1] /= k;
        self[2] /= k;
    }
}
