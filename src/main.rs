mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod libs;
mod interval;
mod camera;

use vec3::Vec3;
use ray::Ray;
use hittable::*;
use sphere::Sphere;
use hittable_list::HittableList;
use interval::Interval;
use camera::Camera;

fn main() -> Result<(), std::io::Error> {
    // world
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.img_width = 400;
    cam.samples_per_pixel = 100;

    cam.render(&world)?;

    Ok(())
}
