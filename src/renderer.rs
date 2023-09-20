use std::{sync::Arc, ops::AddAssign};
use std::ops::Deref;
use std::rc::Rc;
use std::thread;
use crate::types::image::Image;

use crate::{hittable::HittableList, types::{color::RGB, vec3::Vec3}};



pub struct Renderer{
    threads:i32,
    world:Arc<HittableList>,
    camera:Arc<crate::camera::Camera>,
    image:Box<Image>,
    sub_images:Vec<Box<Image>>,
}

impl Renderer {
    pub fn new(threads:i32,world:Arc<HittableList>,camera:Arc<crate::camera::Camera>) -> Renderer{
        Renderer{
            threads,
            world,
            image:Image::new(camera.image_width,camera.image_height),
            camera,
            sub_images:Vec::with_capacity(threads as usize),
        }
    }
    pub fn render(&mut self){
        let (tx, rx) = std::sync::mpsc::channel();

        let samples_per_thread = self.camera.samples_per_pixel/self.threads;
        let last_thread_samples = self.camera.samples_per_pixel - samples_per_thread*(self.threads-1);

        // create threads
        for i in 0..self.threads-1 {
            let tx = tx.clone();
            let camera = Arc::clone(&self.camera);
            let world = Arc::clone(&self.world);
            thread::spawn(move  || {
                eprintln!("thread {} start render",i);
                let sub_image = camera.render(world.deref(),samples_per_thread);
                eprintln!("thread {} end render", i);
                tx.send(sub_image).unwrap();
            });
        }
        let tx = tx.clone();
        let camera = Arc::clone(&self.camera);
        let world = Arc::clone(&self.world);
        let thread_num = self.threads;
        thread::spawn(move  || {
            eprintln!("thread {} start render",thread_num-1);
            let sub_image = camera.render(world.deref(),last_thread_samples);
            eprintln!("thread {} end render", thread_num-1);
            tx.send(sub_image).unwrap();
        });


        for i in 0..self.threads {
            self.sub_images.push(rx.recv().unwrap());
            self.image.add_image(&self.sub_images[i as usize]);
        }

        self.output_image();
    }
    fn output_image(&self){
        print!("P3\n{} {}\n255\n",self.camera.image_width,self.camera.image_height);
        self.image.write(self.camera.samples_per_pixel);
    }
}