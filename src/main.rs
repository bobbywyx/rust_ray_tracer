mod color;
mod vec3;
mod ray;
mod sphere;
mod hittable;
mod interval;
mod camera;
mod random;
mod materials;

use std::sync::Arc;

use crate::vec3::Vec3;
use crate::sphere::Sphere;

use materials::{lambertian::Lambertian,dielectric::Dielectric,metal::Metal};

fn main() {

    // World
    let mut world = hittable::HittableList{objects:Vec::new()};

    let material_ground = Lambertian{albedo:Vec3(0.8,0.8,0.0)};
    let material_center = Lambertian{albedo:Vec3(0.7,0.3,0.3)};
    // let material_center = material::Dielectric::new(0.1);
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Vec3(0.8,0.6,0.2),1.0);


    world.add(Arc::new(Sphere{center:Vec3(0.0,0.0,-1.0),radius:0.5,mat:Box::new(material_center),}));
    world.add(Arc::new(Sphere{center:Vec3(0.0,-100.5,-1.0),radius:100.0,mat:Box::new(material_ground),}));
    world.add(Arc::new(Sphere{center:Vec3(-1.0,0.0,-1.0),radius:0.5,mat:Box::new(material_left),}));
    world.add(Arc::new(Sphere{center:Vec3(1.0,0.0,-1.0),radius:0.5,mat:Box::new(material_right),}));

    // Camera
    let mut camera = camera::Camera::new(16.0/9.0,400,50,50);

    camera.render(&world);
}


fn degrees_to_radians(degrees:f64) -> f64{
    degrees * std::f64::consts::PI / 180.0
}