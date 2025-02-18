pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn deg_to_rad(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn rand_double() -> f64 {
    rand::random_range(0.0..1.0)
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    rand::random_range(min..max)
}
