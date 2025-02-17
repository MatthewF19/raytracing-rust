use crate::hittable::*;

#[derive(Default)]
struct HittableList {
    objects: Vec<Box<dyn Hittable>>, 
}

impl HittableList {
    fn new(object: Box<dyn Hittable>) -> Self {
        Self { objects: vec![object] }
    }

    fn clear(&mut self) {
        self.objects.clear();
    }

    fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest = ray_tmax;

        for object in self.objects.iter() {
            if object.hit(r, ray_tmin, ray_tmax, &mut temp_record) {
                hit_anything = true;
                closest = temp_record.t;
                *rec = temp_record.clone();
            }
        }

        return hit_anything;
    }
}
