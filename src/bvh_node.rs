use std::cmp::Ordering;
use std::rc::Rc;

use crate::aabb::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::vec3::*;
use crate::ray::*;
use crate::interval::*;
use crate::libs::*;
use crate::hittable_list::*;

pub struct BvhNode {
    left:  Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox:  AABB,
}

impl BvhNode {
    pub fn from_hittable_list(mut list: HittableList) -> Self {
        let size = list.objects.len();
        Self::new(&mut list.objects, 0, size)
    }

    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = AABB::empty();
        for obj_idx in start..end {
            bbox = AABB::from_boxes(&bbox, objects[obj_idx].bounding_box());
        }
        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };
        let object_span = end - start;

        let (left, right);
        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start+1].clone();
        } else {
            objects[start..end].sort_by(|a, b| comparator(a.clone(), b.clone()));

            let mid = start + object_span/2;
            left = Rc::new(Self::new(objects, start, mid));
            right = Rc::new(Self::new(objects, mid, end));
        }

        Self { left: left.clone(), right: right.clone(), bbox }
    }

    fn box_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>, axis_idx: i32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_idx);
        let b_axis_interval = b.bounding_box().axis_interval(axis_idx);

        a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
    }

    fn box_x_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}
impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }), rec);

        return hit_left || hit_right;
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
