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
use crate::materials::{lambertian::Lambertian,dielectric::Dielectric,metal::Metal};


fn main() {

    // World
    let mut world = hittable::HittableList{objects:Vec::new()};

    world.random_scene();

    let material1 = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::new(Vec3(0.0,1.0,0.0),1.0,Box::new(material1))));
    let materal2 = Lambertian::new(Vec3(0.4,0.2,0.1));
    world.add(Arc::new(Sphere::new(Vec3(-4.0,1.0,0.0),1.0,Box::new(materal2))));
    let material3 = Metal::new(Vec3(0.7,0.6,0.5),0.0);
    world.add(Arc::new(Sphere::new(Vec3(4.0,1.0,0.0),1.0,Box::new(material3))));



    let lookfrom = Vec3(13.0,2.0,3.0);
    let lookat = Vec3(0.0,0.0,0.0);
    let vup = Vec3(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    // Camera
    let mut camera = camera::Camera::new(20.0,16.0/9.0,700,10,20,&lookfrom,&lookat,&vup,aperture,dist_to_focus);

    camera.render(&world);
}

