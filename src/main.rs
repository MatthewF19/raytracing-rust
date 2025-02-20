mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod libs;
mod interval;
mod camera;
mod material;

use std::rc::Rc;

use vec3::Vec3;
use color::*;
use ray::Ray;
use hittable::*;
use sphere::Sphere;
use hittable_list::HittableList;
use libs::*;
use interval::Interval;
use camera::Camera;
use material::*;

fn main() -> Result<(), std::io::Error> {
    // world
    let mut world = HittableList::default();

    let material_ground = Rc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(&Color::new(0.7, 0.4697, 0.7)));
    let material_left = Rc::new(Dielectric::new(1.5));
    // relative ior: ratio of air to glass
    let material_bubble = Rc::new(Dielectric::new(1.0/1.5));
    let material_right = Rc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(&Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(&Vec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Box::new(Sphere::new(&Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.img_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    cam.lookat = Vec3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world)?;

    Ok(())
}
