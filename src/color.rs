use std::io::Write;

use crate::interval::Interval;

use super::vec3::*;

pub type Color = Vec3;
impl Color {
    fn linear_to_gamma(linear_component: f64) -> f64 {
        match linear_component {
            0.0.. => f64::sqrt(linear_component),
            _   => 0.0
        }
    }

    pub fn write_color(&self, file: &mut dyn Write) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        r = Color::linear_to_gamma(r);
        g = Color::linear_to_gamma(g);
        b = Color::linear_to_gamma(b);

        let intensity = Interval::new(0.0, 0.999);
        let rbyte = (255.999 * intensity.clamp(r)) as i32;
        let gbyte = (255.999 * intensity.clamp(g)) as i32;
        let bbyte = (255.999 * intensity.clamp(b)) as i32;

        let formatted = format!("{} {} {}\n", rbyte, gbyte, bbyte);
        let _ = file.write(formatted.as_bytes());
    }
}
