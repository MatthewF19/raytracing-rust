use std::io::{stderr, stdout, Write};

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod libs;

use hittable::*;
use vec3::Vec3;
use color::Color;
use ray::Ray;
use hittable_list::HittableList;
use sphere::Sphere;

/*
fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - *r.origin();
    // quadratic equation
    let a = r.direction().length_squared();
    let h = r.direction().dot(&oc);
    let c = oc.length_squared() - radius*radius;
    let discriminant = h*h - a*c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - f64::sqrt(discriminant)) / a;
    }
}
*/

fn ray_color<H: Hittable>(r: &Ray, world: &H) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, libs::INFINITY, &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_dir = r.direction().unit_vector();
    let a = 0.5*(unit_dir.y() + 1.0);
    return Color::new(1.0, 1.0, 1.0)*(1.0-a) + Color::new(0.5, 0.7, 1.0)*a;
}

fn main() -> Result<(), std::io::Error> {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;

    // make sure img height is at least 1
    let mut img_height = (img_width as f64 / aspect_ratio) as i32;
    img_height = if img_height < 1 { 1 } else { img_height };

    // world
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // camera settings
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // vectors across horizontal / down vertical edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // distance between pixels
    let pixel_delta_u = viewport_u / img_width as f64;
    let pixel_delta_v = viewport_v / img_height as f64;

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    print!("P3\n{} {}\n255\n", img_width, img_height);

    let mut err = stderr();
    let mut out = stdout();
    for j in 0..img_height {
        let buff = format!("\rScanlines remaining: {} ", img_height-j);
        err.write(buff.as_bytes())?;
        for i in 0..img_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_dir = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_dir);

            let pixel_color = ray_color(&r, &world);
            pixel_color.write_color(&mut out);
        }
    }
    err.write(b"\rDone.                 \n")?;

    Ok(())
}
