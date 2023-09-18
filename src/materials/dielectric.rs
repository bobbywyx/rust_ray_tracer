use crate::{vec3::Vec3, hittable::HitRecord, ray::Ray};
use super::material::Material;

use Vec3 as Color;

pub struct Dielectric {
    pub ir:f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir:f64) -> Dielectric {
        Dielectric{ir:ir}
    }
}

impl Material for Dielectric {
    fn my_copy(&self) -> Box<dyn Material> {
        Box::new(Dielectric{ir:self.ir})
    }

    fn scatter(&self,r_in:&Ray,hit_record:&HitRecord,attenuation:&mut Vec3,scattered:&mut Ray) -> bool {
        attenuation.set(1.0, 1.0, 1.0);
        let refraction_ratio = match hit_record.front_face {
            true => {
                1.0 / self.ir
            }
            false => {
                self.ir
            }
        };

        let unit_dir = r_in.dir.unit_vector();
        let refracted = Vec3::refract(&unit_dir, &hit_record.normal , refraction_ratio);

        // println!("refracted dot r_in dir  : {}",refracted.dot(&r_in.dir));

        scattered.orig = hit_record.p;
        scattered.dir = refracted;
        return true;
    }
}