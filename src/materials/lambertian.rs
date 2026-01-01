use super::material::Material;
use crate::{
    hittable::HitRecord,
    types::{ray::Ray, vec3::Vec3},
};

use Vec3 as Color;
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        // Randomly generating a vector according to Lambertian distribution
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        *scattered = Ray::new_with_tm(hit_record.p, scatter_direction, r_in.time());
        attenuation.clone_from(&self.albedo);
        return true;
    }
    fn my_copy(&self) -> Box<dyn Material + Sync + Send> {
        Box::new(Lambertian {
            albedo: self.albedo,
        })
    }
}
