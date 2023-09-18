use std::sync::Arc;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::materials::material;
use crate::materials::material::Material;
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
