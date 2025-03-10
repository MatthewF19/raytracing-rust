use std::rc::Rc;

use crate::hittable::*;
use crate::material::Material;
use crate::aabb::AABB;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;

pub struct Quad {
    Q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Rc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    D: f64,
}

impl Quad {
    pub fn new(Q: &Vec3, u: &Vec3, v: &Vec3, mat: Rc<dyn Material>) -> Self {
        let bbox = Self::compute_bounding_box(Q, u, v);
        
        // calculate plane of quad
        let n = u.cross(v);
        let normal = n.unit_vector();
        let D = normal.dot(Q);
        let mut w = n / n.dot(&n);
        
        Self { Q: *Q, u: *u, v: *v, w, mat, bbox, normal, D }
    }

    fn compute_bounding_box(Q: &Vec3, u: &Vec3, v: &Vec3) -> AABB {
        let bbox_diag_1 = AABB::from_points(*Q, *Q+*u+*v);
        let bbox_diag_2 = AABB::from_points(*Q+*u, *Q+*v);
        
        AABB::from_boxes(&bbox_diag_1, &bbox_diag_2)
    }

    fn is_interior(a: f64, b: f64) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if (!unit_interval.contains(a)) || (!unit_interval.contains(b)) {
            return false;
        }

        return true;
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&r.direction());

        // ray does not hit if parallel to plane
        if f64::abs(denom) < 1e-3 { return false; }

        // also misses if intersection outside ray bounds
        let t = (self.D - self.normal.dot(&r.origin())) / denom;
        if !ray_t.contains(t) { return false; }

        // check if intersection point within quad
        // (0 <= alpha <= 1 AND 0 <= beta <= 1)
        let intersection = r.at(t);
        let planar_hitpt_vec = intersection - self.Q;
        let alpha = self.w.dot(&planar_hitpt_vec.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vec));

        // ray outside quad
        if !Self::is_interior(alpha, beta) { return false; }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);

        return true;
    }
}