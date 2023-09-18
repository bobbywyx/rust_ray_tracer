use crate::{ray::Ray, hittable::HitRecord, vec3::Vec3};

use Vec3 as Color;

pub trait Material {
    fn scatter(&self,r_in:&Ray,hit_record:&HitRecord,attenuation:&mut Color,scattered:&mut Ray) -> bool;
    fn my_copy(&self) -> Box<dyn Material>;
}

pub struct Nothing{}

impl Material for Nothing {
    fn scatter(&self,r_in:&Ray,hit_record:&HitRecord,attenuation:&mut Color,scattered:&mut Ray) -> bool {
        return false;
    }
    fn my_copy(&self) -> Box<dyn Material> {
        Box::new(Nothing{})
    }
}
