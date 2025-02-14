use std::io::Write;

use super::vec3::*;

pub type Color = Vec3;
impl Color {
    pub fn write_color(&self, file: &mut dyn Write) {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let rbyte = (255.999 * r) as i32;
        let gbyte = (255.999 * g) as i32;
        let bbyte = (255.999 * b) as i32;

        let formatted = format!("{} {} {}\n", rbyte, gbyte, bbyte);
        let _ = file.write(formatted.as_bytes());
    }
}
