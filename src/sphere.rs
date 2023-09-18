use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::materials::material::Material;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere{
    pub center:Vec3,
    pub radius:f64,
    pub mat:Box<dyn Material>,
}

impl Sphere {
    pub fn new(center:Vec3,radius:f64,mat:Box<dyn Material>) -> Sphere {
        Sphere{center:center,radius:radius,mat:mat}
    }
}

pub fn hit_sphere(center:&Vec3,radius:f64,ray:&Ray) -> f64{
    let origin_to_center = ray.orig - *center;
    let a = ray.dir.dot(&ray.dir);
    let half_b = origin_to_center.dot(&ray.dir);
    let c = origin_to_center.dot(&origin_to_center) - radius*radius;
    let discriminant = half_b*half_b - a*c;

    if discriminant<0.0 {
        return -1.0;
    }else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

impl Hittable for Sphere {
    fn hit(&self,r: &Ray,ray_t:&Interval,rec:&mut crate::hittable::HitRecord) -> bool {
        let origin_to_center = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = origin_to_center.dot(&r.dir);
        let c = origin_to_center.length_squared() - self.radius*self.radius;
        
        let discriminant = half_b*half_b - a*c;
    
        if discriminant<0.0 {
            return false;
        }
        
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root){
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root){
                return false;
            }
        }
        
        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;

        // rec.normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.my_copy();
        return true;
    }
}
