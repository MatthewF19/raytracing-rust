use crate::{interval::{self, *}, ray::Ray, vec3::Vec3};

#[derive(Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabb = Self { x, y, z };
        aabb.pad_to_min();
        return aabb;
    }

    pub fn from_points(a: Vec3, b: Vec3) -> Self {
        let x = if a.x() <= b.x() { Interval::new(a.x(), b.x()) } else { Interval::new(b.x(), a.x()) };
        let y = if a.y() <= b.y() { Interval::new(a.y(), b.y()) } else { Interval::new(b.y(), a.y()) };
        let z = if a.z() <= b.z() { Interval::new(a.z(), b.z()) } else { Interval::new(b.z(), a.z()) };

        let mut aabb = Self { x, y, z };
        aabb.pad_to_min();
        return aabb;
    }

    pub fn from_boxes(box0: &AABB, box1: &AABB) -> Self {
        let x = Interval::from_intervals(&box0.x, &box1.x);
        let y = Interval::from_intervals(&box0.y, &box1.y);
        let z = Interval::from_intervals(&box0.z, &box1.z);
        
        let mut aabb = Self { x, y, z };
        aabb.pad_to_min();
        return aabb;
    }

    fn pad_to_min(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta { self.x.expand(delta); }
        if self.y.size() < delta { self.y.expand(delta); }
        if self.z.size() < delta { self.z.expand(delta); }
    }

    pub fn axis_interval(&self, n: i32) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir  = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis as usize];

            let t0 = (ax.min - ray_orig[axis as usize]) * adinv;
            let t1 = (ax.max - ray_orig[axis as usize]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min { ray_t.min = t0; }
                if t1 < ray_t.max { ray_t.max = t1; }
            } else {
                if t1 > ray_t.min { ray_t.min = t1; }
                if t0 < ray_t.max { ray_t.max = t0; }
            }

            if ray_t.max <= ray_t.min { return false; }
        }

        return true;
    }

    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() {
            return if self.x.size() > self.z.size() { 0 } else { 2 };
        } else {
            return if self.y.size() > self.z.size() { 1 } else { 2 };
        }
    }

    pub fn empty() -> Self {
        Self::new(Interval::EMPTY(), Interval::EMPTY(), Interval::EMPTY())
    }

    pub fn universe() -> Self {
        Self::new(Interval::UNIVERSE(), Interval::UNIVERSE(), Interval::UNIVERSE())
    }
}
