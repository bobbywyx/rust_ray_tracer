use crate::hittable::HitRecord;
use crate::types::{ray::Ray, vec3::Vec3};

use Vec3 as Color;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
    fn my_copy(&self) -> Box<dyn Material + Sync + Send>;
}

pub struct Nothing {}

impl Material for Nothing {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Color, _: &mut Ray) -> bool {
        return false;
    }
    fn my_copy(&self) -> Box<dyn Material + Sync + Send> {
        Box::new(Nothing {})
    }
}
