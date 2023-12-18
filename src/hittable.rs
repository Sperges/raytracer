use crate::prelude::*;

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}
