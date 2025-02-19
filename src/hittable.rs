use std::rc::*;

use crate::Ray;
use crate::Vec3;
use crate::Interval;
use crate::material::*;
use crate::color::*;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::default(),
            normal: Vec3::new(1.0, 0.0, 0.0),
            mat: Rc::new(Lambertian::new(&Color::new(0.0, 0.0, 0.0))),
            t: f64::default(),
            front_face: bool::default(),
        }
    }
}

impl HitRecord {
    // outward normal assumed to be unit vector
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
