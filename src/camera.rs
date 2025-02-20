use std::io::{stdout, stderr, Write};

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Hittable;
use crate::HitRecord;
use crate::Interval;
use crate::libs;
use crate::libs::*;
use crate::Metal;

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,

    img_height: i32,
    pixel_samples_scale: f64,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // camera position / rotation
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            img_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Vec3::new(0.0, 0.0, 0.0),
            lookat: Vec3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),

            img_height: i32::default(),
            pixel_samples_scale: f64::default(),
            center: Vec3::default(),
            pixel00_loc: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
        }
    }
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
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j); 
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }

                pixel_color = pixel_color * self.pixel_samples_scale;
                pixel_color.write_color(&mut out);
            }
        }
        err.write(b"\rDone.                 \n")?;

        Ok(())
    }

    fn initialize(&mut self) {
        self.img_height = (self.img_width as f64 / self.aspect_ratio) as i32;
        self.img_height = if self.img_height < 1 { 1 } else { self.img_height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;
        
        // viewport dimensions
        let focal_len = (self.lookfrom - self.lookat).length();
        let theta = deg_to_rad(self.vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h * focal_len;
        let viewport_width = viewport_height * (self.img_width as f64 / self.img_height as f64);

        // basis vecs for camera coordinates
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // vectors across horiz / vert of screen
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.img_width as f64;
        self.pixel_delta_v = viewport_v / self.img_height as f64;

        let viewport_upper_left = self.center - (focal_len * self.w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v)*0.5
    }

    // get ray originating from origin pointed at random point around i, j
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc + 
                           (self.pixel_delta_u * (i as f64 + offset.x())) +
                           (self.pixel_delta_v * (j as f64 + offset.y()));

        let ray_origin = self.center;
        let ray_dir = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_dir);
    }

    // random point in [-.5, -.5] - [.5, .5] unit square
    fn sample_square() -> Vec3 {
        Vec3::new(rand_double() - 0.5, rand_double() + 0.5, 0.0)
    }

    fn ray_color<H: Hittable>(r: &Ray, depth: i32, world: &H) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0); 
        }

        let mut rec = HitRecord::default();

        if world.hit(r, Interval::new(0.001, libs::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(&scattered, depth-1, world);
            }

            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_dir = r.direction().unit_vector();
        let a = 0.5*(unit_dir.y() + 1.0);
        return Color::new(1.0, 1.0, 1.0)*(1.0-a) + Color::new(0.5, 0.7, 1.0)*a;
    }
}
