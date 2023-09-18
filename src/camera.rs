use std::f64::INFINITY;

use crate::hittable::{self, Hittable};
use crate::{interval, random};
use crate::{ray::Ray, hittable::HittableList, vec3::Vec3};

use crate::vec3::Vec3 as Point3;

use crate::color::write_color;

pub struct Camera{
    pub aspect_ratio:f64,
    pub image_width:i32,
    pub samples_per_pixel:i32,
    pub max_depth:i32,
    image_height:i32,
    center:Point3,
    pixel00_loc:Point3,
    pixel_delta_u:Vec3,
    pixel_delta_v:Vec3,
}

impl Camera {

    pub fn new(aspect_ratio:f64,image_width:i32,samples_per_pixel:i32,max_depth:i32) -> Camera{
        Camera{
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height:0,
            center:Point3(0.0,0.0,0.0),
            pixel00_loc:Point3(0.0,0.0,0.0),
            pixel_delta_u:Vec3(0.0,0.0,0.0),
            pixel_delta_v:Vec3(0.0,0.0,0.0),
        }
    }

    pub fn render(&mut self,world:&HittableList){
        self.init();
        print!("P3\n{} {}\n255\n",self.image_width,self.image_height);

        let mut j = 0;
        while j<self.image_height {
            let mut i = 0;
            while i<self.image_width {
                use Vec3 as Color;
                let mut pixel_color = Color(0.0,0.0,0.0);
                
                for _ in 0..self.samples_per_pixel{
                    let ray = self.get_ray(i, j);
                    pixel_color = pixel_color + ray_color(&ray,self.max_depth,&world);
                }

                // let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                // let ray_direction = pixel_center - self.center;
                // let r = Ray{orig: self.center, dir: ray_direction};

                // let pixel_color = ray_color(&r,&world);

                write_color(pixel_color,self.samples_per_pixel);

                i+=1;
            }
            j += 1;
        }

    }

    fn init(&mut self){    
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        
        if self.image_height == 0 {
            self.image_height = 1;
        }

        self.center = Point3(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height as f64 * ((self.image_width as f64 )/self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);


        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;


        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center
            - Vec3(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;

        self.pixel00_loc = viewport_upper_left +  (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

    }

    fn pixel_center_square(&self) -> Vec3{
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + random::random_f64();
        let py = -0.5 + random::random_f64();

        // print!("{} {}\n",px,py);

        return (self.pixel_delta_u * px) + (self.pixel_delta_v * py);
    }

    fn get_ray(&self,u:i32,v:i32) -> Ray{
        // Get a randomly sampled camera ray for the pixel at location i,j.
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * u as f64) + (self.pixel_delta_v * v as f64);
        let pixel_sample  = pixel_center + self.pixel_center_square();

        let ray_origin = self.center;
        let ray_dir = pixel_sample - ray_origin;

        return Ray{orig:ray_origin,dir:ray_dir};
    }

}

fn ray_color(ray:&Ray,depth:i32,world:&HittableList) -> Vec3{
    use Vec3 as Color;

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color(0.0,0.0,0.0);
    }

    let mut rec = hittable::HitRecord::new();
    if(world.hit(ray, &interval::Interval { min: 0.001, max: INFINITY }, &mut rec)){
        let mut scattered:Ray = Ray::new(Vec3::zero_vec3(), Vec3::zero_vec3());
        let mut attenuation:Color = Color(0.0,0.0,0.0);

        if rec.mat.scatter(ray, &rec, &mut attenuation, &mut scattered){
            return attenuation * ray_color(&scattered,depth-1,world);
        }
        
        return Color(0.0,0.0,0.0);

        // old

        // // let dir = Vec3::random_on_hemisphere(&rec.normal);
        // let dir = Vec3::random_unit_vector() + rec.normal;
        // return ray_color(&Ray { orig: rec.p, dir: dir },depth-1,world) * 0.5;

        // // return (rec.normal + color(1.0,1.0,1.0)) * 0.5;
        // old end
    }
    
    let unit_direction = ray.dir.unit_vector();
    let a = 0.5*(unit_direction.y() + 1.0);
    Color(1.0, 1.0, 1.0) * (1.0-a) + Color(0.5, 0.7, 1.0) * a
}
