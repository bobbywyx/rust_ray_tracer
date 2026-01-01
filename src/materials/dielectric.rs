use super::material::Material;
use crate::{
    hittable::HitRecord,
    random,
    types::{ray::Ray, vec3::Vec3},
};

pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir: ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for Dielectric {
    fn my_copy(&self) -> Box<dyn Material + Sync + Send> {
        Box::new(Dielectric { ir: self.ir })
    }

    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        attenuation.set(1.0, 1.0, 1.0);
        let refraction_ratio = match hit_record.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };

        let unit_dir = r_in.dir().unit_vector();
        let cos_theta = (-unit_dir).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = match cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random::random_f64()
        {
            true => unit_dir.reflect(&hit_record.normal),
            false => Vec3::refract(&unit_dir, &hit_record.normal, refraction_ratio),
        };
        *scattered = Ray::new(hit_record.p, direction);
        return true;
    }
}
