use crate::{types::{ray::Ray, vec3::Vec3}, hittable::HitRecord};
use super::material::Material;


use Vec3 as Color;
pub struct Lambertian{
    pub albedo:Color,
}

impl Lambertian {
    pub fn new(albedo:Color) -> Lambertian {
        Lambertian{albedo:albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self,r_in:&Ray,hit_record:&HitRecord,attenuation:&mut Vec3,scattered:&mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        scattered.orig = hit_record.p;
        scattered.dir = scatter_direction;
        attenuation.clone_from(&self.albedo);
        return true;
    }
    fn my_copy(&self) -> Box<dyn Material> {
        Box::new(Lambertian{albedo:self.albedo})
    }
}