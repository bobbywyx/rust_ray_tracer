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
    

pub struct Lambertian{
    pub albedo:Color,
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

pub struct Metal{
    pub albedo:Color,
    pub fuzz:f64,
}

impl Metal {
    pub fn new(albedo:Color,fuzz:f64) -> Metal {
        let mut f = fuzz;
        if f>1.0 {
            f = 1.0;
        }
        Metal{albedo:albedo,fuzz:f}
    }
}

impl Material for Metal {
    fn my_copy(&self) -> Box<dyn Material> {
        Box::new(Metal{albedo:self.albedo,fuzz:self.fuzz})
    }
    fn scatter(&self,r_in:&Ray,hit_record:&HitRecord,attenuation:&mut Vec3,scattered:&mut Ray) -> bool {
        let reflected = r_in.dir.unit_vector().reflect(&hit_record.normal);

        scattered.orig = hit_record.p;
        scattered.dir = reflected + self.fuzz * Vec3::random_in_unit_sphere();
        
        attenuation.clone_from(&self.albedo);
        return scattered.dir.dot(&hit_record.normal) > 0.0;
    }
}

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

        scattered.orig = hit_record.p;
        scattered.dir = refracted;
        return true;
    }
}