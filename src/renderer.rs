use std::fs::File;
use std::io::Write;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::ops::Deref;
use std::thread;
use crate::types::image::Image;

use crate::types::render_task::RenderTask;
use crate::hittable::HittableList;



pub struct Renderer{
    threads:i32,
    world:Arc<HittableList>,
    camera:Arc<crate::camera::Camera>,
    image:Box<Image>,
    sub_images:Vec<Box<Image>>,
    current_task_id:i32,
    file:File
}

impl Renderer {
    pub fn new(threads:i32,world:Arc<HittableList>,camera:Arc<crate::camera::Camera>,f:File) -> Renderer{
        Renderer{
            threads,
            world,
            image:Image::new(camera.image_width,camera.image_height),
            camera,
            sub_images:Vec::with_capacity(threads as usize),
            current_task_id:0,
            file:f
        }
    }

    pub fn generate_render_tasks(&self) -> Vec<RenderTask>{
        let task_nums = self.threads * self.threads;
        let mut task_queue:Vec<RenderTask> = Vec::with_capacity(task_nums as usize);

        let samples_per_thread = self.camera.samples_per_pixel/self.threads;
        let last_thread_samples = self.camera.samples_per_pixel - samples_per_thread*(self.threads-1);

        let pixel_nums = self.camera.image_width*self.camera.image_height;

        let pixels_per_thread = pixel_nums/self.threads;
        let last_thread_pixels = pixel_nums - pixels_per_thread*(self.threads-1);


        // 将一个图片分成多个部分，同时多个部分的采样数也不一样
        // i 代表部分，j代表采样
        for sample_part in 0..self.threads {
            for img_part in 0..self.threads {
                let start_id;
                let end_id ;
                let s ;
                if img_part == self.threads-1 {
                    start_id = img_part * pixels_per_thread;
                    end_id = start_id + last_thread_pixels;
                } else {
                    start_id = img_part*pixels_per_thread;
                    end_id = start_id + pixels_per_thread;
                }
                if sample_part == self.threads-1 {
                    s = last_thread_samples;
                } else {
                    s = samples_per_thread;
                }
                task_queue.push(RenderTask::new(start_id,end_id,s));
            }
        }
        return task_queue;
    }

    pub fn render(&mut self){
        let (tx, rx) = std::sync::mpsc::channel();
        let task_nums = self.threads * self.threads;

        let task_queue = self.generate_render_tasks();

        // create threads
        for i in 0..self.threads {
            self.create_one_thread(tx.clone(),task_queue[self.current_task_id as usize],i);
        }


        for i in 0..task_nums {
            self.sub_images.push(rx.recv().unwrap());

            if self.current_task_id < task_nums {
                // 只剩下最后几个任务了，不需要再创建线程了
                self.create_one_thread(tx.clone(),task_queue[self.current_task_id as usize],i%self.threads);
            }
            self.image.add_image(&self.sub_images[i as usize]);
        }

        self.output_image();
    }
    fn output_image(&mut self){
        // print!("P3\n{} {}\n255\n",self.camera.image_width,self.camera.image_height);
        self.file.write_fmt(format_args!("P3\n{} {}\n255\n",self.camera.image_width,self.camera.image_height)).unwrap();
        self.image.write(self.camera.samples_per_pixel,&self.file);
    }

    fn create_one_thread(&mut self,tx:Sender<Box<Image>>,task:RenderTask,id:i32){
        self.current_task_id+=1;
        let camera = Arc::clone(&self.camera);
        let world = Arc::clone(&self.world);

        thread::spawn(move  || {
            println!("thread {} start render",id);
            println!("task pixels :{} and samples :{}",task.end_id - task.start_id,task.samples_per_pixel);
            let sub_image = camera.render(world.deref(),task);
            println!("thread {} end render", id);
            tx.send(sub_image).unwrap();
        });
    }
}