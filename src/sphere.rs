use crate::hittable::*;
use crate::ray::Ray;
use crate::Vec3;
use crate::interval::*;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64) -> Self {
        Self { center: *center, radius: radius.max(0.0) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        // quadratic equation
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        // find nearest root in acceptable range
        let mut root = (h - sqrtd) / a;
        if !(ray_t.surrounds(root)) {
            root = (h + sqrtd) / a;
            if !(ray_t.surrounds(root)) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        return true;
    }
}
