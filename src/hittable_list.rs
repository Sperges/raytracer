use crate::prelude::*;

// TODO: find a better way of handling lists in rust; section 6.5

pub struct HittableList {
    spheres: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
        }
    }

    pub fn clear_spheres(&mut self) {
        self.spheres.clear();
    }

    pub fn add_sphere(&mut self, object: Sphere) {
        self.spheres.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.spheres.iter() {
            let mut temp_rec = HitRecord::new();
            if object.hit(ray, t_min, t_max, &mut temp_rec) {
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