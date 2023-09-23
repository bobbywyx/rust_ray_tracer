use crate::hittable::{self, Hittable, HittableList};
use crate::random::random_f64_with_bounds;
use crate::types::interval;
use crate::types::ray::Ray;
use crate::types::render_task::RenderTask;
use crate::types::vec3::Vec3;

use crate::types::vec3::Vec3 as Point3;

use crate::types::image::Image;

pub struct Camera{
    pub vfov:f64,
    pub aspect_ratio:f64,
    pub image_width:i32,
    pub samples_per_pixel:i32,
    pub max_depth:i32,
    pub image_height:i32,
    center:Point3,
    lower_left_corner:Point3,
    horizontal:Vec3,
    vertical:Vec3,
    u:Vec3,
    v:Vec3,
    w:Vec3,
    lens_radius:f64,
}

impl Camera {
    pub fn new(vfov:f64,aspect_ratio:f64,image_width:i32,samples_per_pixel:i32,max_depth:i32,lookfrom:&Vec3,lookat:&Vec3,vup:&Vec3,aperture:f64,focus_dist:f64) -> Camera{
        let mut camera = Camera{
            vfov,
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height:0,
            center:Point3(0.0,0.0,0.0),
            lower_left_corner:Point3(0.0,0.0,0.0),
            horizontal:Vec3(0.0,0.0,0.0),
            vertical:Vec3(0.0,0.0,0.0),
            u:Vec3(0.0,0.0,0.0),
            v:Vec3(0.0,0.0,0.0),
            w:Vec3(0.0,0.0,0.0),
            lens_radius:0.0
        };
        camera.init(lookfrom,lookat,vup,aperture,focus_dist);
        return camera;
    }

    pub fn render(&self,world:&HittableList,task:RenderTask) -> Box<Image>{
        let mut j = 0;
        let mut image = Image::new(self.image_width,self.image_height);
        let sample_num = task.samples_per_pixel;

        while j<self.image_height {
            let mut i = 0;
            // while i<self.image_width && j*self.image_width+i < task.end_id && task.start_id <= j*self.image_width+i {
            loop{
                if task.start_id <= j*self.image_width+i {
                use Vec3 as Color;
                let mut pixel_color = Color(0.0,0.0,0.0);
                
                for _ in 0..sample_num{
                    let ray = self.get_ray(i as f64 / self.image_width as f64, (self.image_height -  j) as f64 / self.image_height as f64);
                    pixel_color = pixel_color + ray_color(&ray,self.max_depth,&world);
                }
                image.set_pixel(i,j,pixel_color);
                // write_color(pixel_color,self.samples_per_pixel);
                }
                i+=1;
                if i>=self.image_width || j*self.image_width+i >= task.end_id{
                    break;
                }
            }
            if j*self.image_width >= task.end_id {
                break;
            }
            j += 1;
        }
        image
    }

    fn init(&mut self,lookfrom:&Vec3,lookat:&Vec3,vup:&Vec3,aperture:f64,focus_dist:f64){    
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        
        if self.image_height == 0 {
            self.image_height = 1;
        }

        let theta = degrees_to_radians(self.vfov);
        let h = (theta/2.0).tan();

        // Determine viewport dimensions.
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        self.w = (*lookfrom - *lookat).unit_vector();
        self.u = vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        self.center = lookfrom.clone();
        self.horizontal = focus_dist * viewport_width * self.u;
        self.vertical = focus_dist * viewport_height * self.v;
        self.lower_left_corner = self.center - self.horizontal/2.0 - self.vertical/2.0 - focus_dist * self.w;
        
        self.lens_radius = aperture / 2.0;

        // println!("lower left corner {:?}",self.lower_left_corner);
        // println!("up right corner {:?}",self.lower_left_corner + self.horizontal + self.vertical);

    }


    fn get_ray(&self,s:f64,t:f64) -> Ray{
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        return Ray{orig:self.center + offset,
                    dir:self.lower_left_corner + s * self.horizontal + t *self.vertical - self.center - offset};
    }

}

fn ray_color(ray:&Ray,depth:i32,world:&HittableList) -> Vec3{
    use Vec3 as Color;

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color(0.0,0.0,0.0);
    }

    let mut rec = hittable::HitRecord::new();
    if world.hit(ray, &interval::Interval { min: 0.001, max: f64::INFINITY }, &mut rec) {
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

fn degrees_to_radians(degrees:f64) -> f64{
    degrees * std::f64::consts::PI / 180.0
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_f64_with_bounds(-1.0,1.0),random_f64_with_bounds(-1.0, 1.0),0.0);
        if p.length_squared() >=1.0 {continue;}
        return p;
    }
}