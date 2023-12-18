use std::vec;

use crate::prelude::*;

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<T>(&mut self, object: T) where T: Hittable + 'static {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            let mut temp_rec = HitRecord::new();
            if object.hit(ray, &Interval::from_f64(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
            }
        }

        hit_anything
    }
}