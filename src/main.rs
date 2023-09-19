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

use materials::{lambertian::Lambertian,dielectric::Dielectric,metal::Metal, material::Material};

fn main() {
    // World
    let mut world = hittable::HittableList{objects:Vec::new()};

    let material_ground = Lambertian{albedo:Vec3(0.8,0.8,0.0)};
    let material_center = Lambertian{albedo:Vec3(0.1,0.2,0.5)};
    // let material_center = material::Dielectric::new(0.1);
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Vec3(0.8,0.6,0.2),1.0);


    world.add(Arc::new(Sphere::new(Vec3(0.0,0.0,-1.0),0.5,Box::new(material_center))));
    world.add(Arc::new(Sphere::new(Vec3(0.0,-100.5,-1.0),100.0,Box::new(material_ground))));
    world.add(Arc::new(Sphere::new(Vec3(-1.0,0.0,-1.0),0.5,material_left.my_copy())));
    world.add(Arc::new(Sphere::new(Vec3(-1.0,0.0,-1.0),-0.4,Box::new(material_left))));
    world.add(Arc::new(Sphere::new(Vec3(1.0,0.0,-1.0),0.5,Box::new(material_right))));
    

    // let R = (std::f64::consts::PI / 4.0).cos();
    // let material_left = Lambertian::new(Vec3(0.0,0.0,1.0));
    // let material_right = Lambertian::new(Vec3(1.0,0.0,0.0));

    // world.add(Arc::new(Sphere::new(Vec3(-R,0.0,-1.0),R,Box::new(material_left))));
    // world.add(Arc::new(Sphere::new(Vec3(R,0.0,-1.0),R,Box::new(material_right))));  


    let lookfrom = Vec3(3.0,3.0,2.0);
    let lookat = Vec3(0.0,0.0,-1.0);
    let vup = Vec3(0.0,1.0,0.0);
    let dist_to_focus = (lookfrom-lookat).length();
    let aperture = 2.0;

    // Camera
    let mut camera = camera::Camera::new(20.0,16.0/9.0,400,50,50,&lookfrom,&lookat,&vup,aperture,dist_to_focus);

    camera.render(&world);
}
