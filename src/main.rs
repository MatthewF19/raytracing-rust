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
mod aabb;
mod bvh_node;
mod quad;

use std::io::{stderr, Write};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

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
use bvh_node::BvhNode;
use quad::Quad;

#[show_image::main]
fn main() -> Result<(), std::io::Error> {
    // world
    let mut world = HittableList::default();

    /*
    let material_ground = Rc::new(Lambertian::new(&Color::new(1.0, 0.0, 1.0)));
    let material_left =   Rc::new(Dielectric::new(1.51));
    let material_center = Rc::new(Dielectric::new(1.51));
    let material_right =  Rc::new(Dielectric::new(1.51));

    world.add(Rc::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(&Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(&Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));
    */

    /*
    let ground_mat = Rc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(&Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat)));

    for a in -11..11 {
        for b in -11..11 {
            let mat = rand_double();
            let center = Vec3::new(a as f64 + 0.9*rand_double(), 0.2, b as f64 + 0.9*rand_double());
            
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian::new(&albedo));
                    world.add(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
                } else if mat < 0.95 {
                    let albedo = Color::random_bounded(0.5, 1.0);
                    let fuzz = rand_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(&albedo, fuzz));
                    world.add(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
                } else {
                    sphere_material = Rc::new(Dielectric::new(1.333, 0.0));
                    world.add(Rc::new(Sphere::new(&center, 0.2, sphere_material)));
                }
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.333, 0.1));
    world.add(Rc::new(Sphere::new(&Vec3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    // let mat2 = Rc::new(Lambertian::new(&Color::new(0.7, 0.4697, 0.7)));
    let mat2 = Rc::new(Dielectric::new(1.333, 0.2));
    world.add(Rc::new(Sphere::new(&Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    // let mat3 = Rc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.1));
    let mat3 = Rc::new(Dielectric::new(1.333, 0.01));
    world.add(Rc::new(Sphere::new(&Vec3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    world = HittableList::new(Rc::new(BvhNode::from_hittable_list(world)));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.img_width = 400;
    cam.samples_per_pixel = 32;
    cam.max_depth = 8;

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world)?;
    */

    let left_red =     Rc::new(Lambertian::new(&Color::new(1.0, 0.2, 0.2)));
    let back_green =   Rc::new(Lambertian::new(&Color::new(0.2, 1.0, 0.2)));
    let right_blue =   Rc::new(Lambertian::new(&Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::new(&Color::new(1.0, 0.5, 0.0)));
    let lower_teal =   Rc::new(Lambertian::new(&Color::new(0.2, 0.8, 0.8)));

    world.add(Rc::new(Quad::new(&Vec3::new(-3.0, -2.0, 5.0), 
                                &Vec3::new(0.0, 0.0, -4.0),
                                &Vec3::new(0.0, 4.0, 0.0),
                                left_red)));
    world.add(Rc::new(Quad::new(&Vec3::new(-2.0, -2.0, 0.0), 
                                &Vec3::new(4.0, 0.0, 0.0),
                                &Vec3::new(0.0, 4.0, 0.0),
                                back_green)));
    world.add(Rc::new(Quad::new(&Vec3::new(3.0, -2.0, 1.0), 
                                &Vec3::new(0.0, 0.0, 4.0),
                                &Vec3::new(0.0, 4.0, 0.0),
                                right_blue)));
    world.add(Rc::new(Quad::new(&Vec3::new(-2.0, 3.0, 1.0), 
                                &Vec3::new(4.0, 0.0, 0.0),
                                &Vec3::new(0.0, 0.0, 4.0),
                                upper_orange)));
    world.add(Rc::new(Quad::new(&Vec3::new(-2.0, -3.0, 5.0), 
                                &Vec3::new(4.0, 0.0, 0.0),
                                &Vec3::new(0.0, 0.0, -4.0),
                                lower_teal)));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.img_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 80.0;
    cam.lookfrom = Vec3::new(0.0, 0.0, 9.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    world = HittableList::new(Rc::new(BvhNode::from_hittable_list(world)));

    cam.render(&world);

    // printing done in render because loop occurs there
    // stderr().write_all(format!{"execution took: {:?}\n", end_time - start_time}.as_bytes())?;

    Ok(())
}
