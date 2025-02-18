use std::io::{stdout, stderr, Write};

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Hittable;
use crate::HitRecord;
use crate::Interval;
use crate::libs;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: i32,
    img_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render<H: Hittable>(&mut self, world: &H) -> Result<(), std::io::Error> {
        self.initialize(); 

        print!("P3\n{} {}\n255\n", self.img_width, self.img_height);

        let mut err = stderr();
        let mut out = stdout();
        for j in 0..self.img_height {
            let buff = format!("\rScanlines remaining: {} ", self.img_height-j);
            err.write(buff.as_bytes())?;
            for i in 0..self.img_width {
                let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                let ray_dir = pixel_center - self.center;
                let r = Ray::new(self.center, ray_dir);

                let pixel_color = Camera::ray_color(&r, world);
                pixel_color.write_color(&mut out);
            }
        }
        err.write(b"\rDone.                 \n")?;

        Ok(())
    }

    fn initialize(&mut self) {
        self.img_height = (self.img_width as f64 / self.aspect_ratio) as i32;
        self.img_height = if self.img_height < 1 { 1 } else { self.img_height };

        self.center = Vec3::new(0.0, 0.0, 0.0);
        
        // viewport dimensions
        let focal_len = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.img_width as f64 / self.img_height as f64);

        // vectors across horiz / vert of screen
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.img_width as f64;
        self.pixel_delta_v = viewport_v / self.img_height as f64;

        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_len) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v)*0.5
    }

    fn ray_color<H: Hittable>(r: &Ray, world: &H) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, libs::INFINITY), &mut rec) {
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_dir = r.direction().unit_vector();
        let a = 0.5*(unit_dir.y() + 1.0);
        return Color::new(1.0, 1.0, 1.0)*(1.0-a) + Color::new(0.5, 0.7, 1.0)*a;
    }
}
