use std::rc::Rc;

use crate::hittable::*;
use crate::interval::*;
use crate::aabb::*;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>, 
    pub bbox:    AABB,
}

impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        Self { objects: vec![object], bbox: AABB::default() }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = AABB::from_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.min, closest), &mut temp_record) {
                hit_anything = true;
                closest = temp_record.t;
                *rec = temp_record.clone();
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
