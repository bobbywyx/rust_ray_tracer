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

        let camera = Arc::clone(&self.camera);
        let world = Arc::clone(&self.world);

        // 创建线程，并发送消息
        thread::spawn(move  || {
            // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
            // let mut sub_image1 =Box::new(Image::new(camera.image_width, camera.image_height));
            // let world_obj_nums = world.objects.iter().count();
            // sub_image1.set_pixel(camera.image_width-1,0,Vec3::new(1.2,3.4,world_obj_nums as f64));

            eprint!("thread start render");
            let sub_image2 = camera.render(world.deref());
            eprint!("thread end render");

            tx.send(sub_image2).unwrap();

            // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
            // tx.send(Some(1)).unwrap()
        });

        // eprintln!("pixel from thread1:{}",rx.recv().unwrap().pixels[0][self.camera.image_width as usize -1].2);
        self.sub_images.push(rx.recv().unwrap());

        eprintln!("pixel from thread1:{}",self.sub_images[0].pixels[0][self.camera.image_width as usize -1].2);


        self.image.add_image(&self.sub_images[0]);

        self.output_image();
    }
    fn output_image(&self){
        print!("P3\n{} {}\n255\n",self.camera.image_width,self.camera.image_height);
        self.image.write(self.camera.samples_per_pixel);
    }
}