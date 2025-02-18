use crate::libs::*;

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self {e: [e0, e1, e2]}
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }        

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] +
        self.e[1] * self.e[1] +
        self.e[2] * self.e[2]
    }

    pub fn unit_vector(&self) -> Self {
        return *self / self.length();
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random();
            let len_sq = p.length_squared();
            // 1e-50 to account for floating point errors
            if 1e-50 < len_sq && len_sq <= 1.0 {
                return p / f64::sqrt(len_sq); 
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        return self.e[0]*other.e[0] +
               self.e[1]*other.e[1] +
               self.e[2]*other.e[2];
    }

    pub fn random() -> Self {
        Self::new(rand_double(), rand_double(), rand_double())
    }

    pub fn random_bounded(min: f64, max: f64) -> Self {
        Self::new(rand_range(min, max), rand_range(min, max), rand_range(min, max))
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {e: [-self.e[0], -self.e[1], -self.e[2]]}
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index] 
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {e: [self.e[0] + rhs.e[0],
                  self.e[1] + rhs.e[1],
                  self.e[2] + rhs.e[2]]}
    }
}
impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {e: [self.e[0] - rhs.e[0],
                  self.e[1] - rhs.e[1],
                  self.e[2] - rhs.e[2]]}
    }
}
impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

// scale by constant
impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {e: [self.e[0]*rhs,
                  self.e[1]*rhs,
                  self.e[2]*rhs]}
    }
}
// scaling is commutative
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.e[0]*self,
                  rhs.e[1]*self,
                  rhs.e[2]*self) 
    }
}
impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

// cross product
impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {e: [self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                  self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                  self.e[0] * rhs.e[1] - self.e[1] * rhs.e[1]]}
    }
}
impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs;
    } 
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {e: [self.e[0]/rhs,
                  self.e[1]/rhs,
                  self.e[2]/rhs]}
    }
}
impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
