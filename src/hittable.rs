use std::sync::Arc;

use crate::materials::material;
use crate::materials::material::Material;
use crate::types::interval::Interval;
use crate::types::ray::Ray;
use crate::types::vec3::Vec3;
use Vec3 as Point3;

// #[derive(Copy,Clone)]
pub struct HitRecord{
    pub p:Point3,
    pub normal:Vec3,
    pub mat: Box<dyn Material>,
    pub t:f64,
    pub front_face : bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self,r:&Ray,outward_normal:&Vec3){
        self.front_face = r.dir().dot(outward_normal) < 0.0;
        // println!("dot product {}",r.dir().dot(outward_normal));
        // print!("if front face {}\n",self.front_face);
        self.normal = match self.front_face {
            true => {
                *outward_normal
            }
            false => {
                -*outward_normal
            }
        }
    }
    pub fn new() -> HitRecord{
        HitRecord{
            p:Point3(0.0,0.0,0.0),
            normal:Vec3(0.0,0.0,0.0),
            mat:Box::new(material::Nothing{}),
            t:0.0,
            front_face:true
        }
    }
}

pub trait Hittable {
    fn hit(&self,r: &Ray,ray_t:&Interval,rec:&mut HitRecord) -> bool;    
}


pub struct HittableList{
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self){
        self.objects.clear();
    }
    pub fn add(&mut self,object:Arc<dyn Hittable>){
        self.objects.push(object);
    }
    pub fn random_scene(&mut self){
        use crate::materials::{lambertian::Lambertian,dielectric::Dielectric,metal::Metal, material::Material};
        use crate::sphere::Sphere;
        use crate::random::{random_f64_with_bounds,random_f64};
        use Vec3 as Color;

        let material_ground = Lambertian{albedo:Vec3(0.5,0.5,0.5)};
        self.add(Arc::new(Sphere::new(Vec3(0.0,-1000.0,-1.0),1000.0,Box::new(material_ground))));
    
        let mut a = -11;
        while a < 11 {
            let mut b = -11;
            while b < 11 {
                let choose_mat = random_f64();    
                let center = Point3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64 +0.9*random_f64());
    
                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let mut sphere_material: Box<dyn Material>;
    
                    if choose_mat < 0.8 {
                        // diffuse
                        let aldebo = Color::random() * Color::random();
                        sphere_material = Box::new(Lambertian::new(aldebo));
                        self.add(Arc::new(Sphere::new(center,0.2,sphere_material)));
                    } else if choose_mat < 0.95 {
                        // metal
                        let aldebo = Color::random_with_bounds(0.5,1.0);
                        let fuzz = random_f64_with_bounds(0.0,0.5);
                        sphere_material = Box::new(Metal::new(aldebo,fuzz));
                        self.add(Arc::new(Sphere::new(center,0.2,sphere_material)));
                    } else {
                        // glass
                        sphere_material = Box::new(Dielectric::new(1.5));
                        self.add(Arc::new(Sphere::new(center,0.2,sphere_material)));
                    }
                }
                b+=1;
            }
            a+=1;
        }


    }
}

impl Hittable for HittableList {
    fn hit(&self,r: &Ray,ray_t:&Interval,rec:&mut HitRecord) -> bool{
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in self.objects.iter() {
            let mut temp_rec = HitRecord::new();
            if(object.hit(r, &Interval{min:ray_t.min,max:closest_so_far}, &mut temp_rec)){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }
}
