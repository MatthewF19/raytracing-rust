pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn deg_to_rad(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}
