use std::rc::*;

use crate::hittable::*;
use crate::ray::Ray;
use crate::Vec3;
use crate::interval::*;
use crate::material::*;
use crate::color::*;
use crate::aabb::AABB;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self { center: *center, radius: radius.max(0.0), mat, 
               bbox: AABB::from_points(*center-rvec, *center+rvec) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        // quadratic equation
        let r_dir = r.direction();
        let a = r_dir.length_squared();
        let h = r_dir.dot(&oc);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        // find nearest root in acceptable range
        let inv_a = 1.0 / a;
        let mut root = (h - sqrtd) * inv_a;
        if !(ray_t.surrounds(root)) {
            root = (h + sqrtd) * inv_a;
            if !(ray_t.surrounds(root)) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
