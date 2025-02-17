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

    pub fn dot(&self, other: &Self) -> f64 {
        return self.e[0]*other.e[0] +
               self.e[1]*other.e[1] +
               self.e[2]*other.e[2];
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

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {e: [self.e[0] - rhs.e[0],
                  self.e[1] - rhs.e[1],
                  self.e[2] - rhs.e[2]]}
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
// cross product
impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {e: [self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                  self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                  self.e[0] * rhs.e[1] - self.e[1] * rhs.e[1]]}
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
