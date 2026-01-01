use super::material::Material;
use crate::{
    hittable::HitRecord,
    types::{ray::Ray, vec3::Vec3},
};

use Vec3 as Color;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        let mut f = fuzz;
        if f > 1.0 {
            f = 1.0;
        }
        Metal {
            albedo: albedo,
            fuzz: f,
        }
    }
}

impl Material for Metal {
    fn my_copy(&self) -> Box<dyn Material + Sync + Send> {
        Box::new(Metal {
            albedo: self.albedo,
            fuzz: self.fuzz,
        })
    }
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.dir().unit_vector().reflect(&hit_record.normal);

        *scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        attenuation.clone_from(&self.albedo);
        return scattered.dir().dot(&hit_record.normal) > 0.0;
    }
}
