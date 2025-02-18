use std::io::Write;

use crate::interval::Interval;

use super::vec3::*;

pub type Color = Vec3;
impl Color {
    pub fn write_color(&self, file: &mut dyn Write) {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let intensity = Interval::new(0.0, 0.999);
        let rbyte = (256.0 * intensity.clamp(r)) as i32;
        let gbyte = (256.0 * intensity.clamp(g)) as i32;
        let bbyte = (256.0 * intensity.clamp(b)) as i32;

        let formatted = format!("{} {} {}\n", rbyte, gbyte, bbyte);
        let _ = file.write(formatted.as_bytes());
    }
}
