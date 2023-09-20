use std::ops::AddAssign;
use crate::types::color::write_color;
use crate::types::vec3::Vec3;

pub struct Image{
    pub(crate) width:i32,
    pub(crate) height:i32,
    pub(crate) pixels:Vec<Vec<Vec3>>,
}

impl Image {
    pub fn new(width:i32,height:i32) -> Box<Image>{
        let mut pixels = Vec::with_capacity(height as usize);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width as usize);
            for _ in 0..width {
                row.push(Vec3(0.0,0.0,0.0));
            }
            pixels.push(row);
        }
        Box::new(Image{
            width,
            height,
            pixels,
        })
    }
    pub fn set_pixel(&mut self,x:i32,y:i32,color:Vec3){
        self.pixels[y as usize][x as usize] = color;
    }

    pub fn add_image(&mut self, other:&Box<Image>) -> &mut Image {
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixels[y as usize][x as usize] += other.pixels[y as usize][x as usize];
            }
        }
        self
    }

    pub fn write(&self,samples_per_pixel:i32){
        for y in 0..self.height {
            for x in 0..self.width {
                write_color(self.pixels[y as usize][x as usize],samples_per_pixel);
            }
        }
    }

}

impl AddAssign for Image {
    fn add_assign(&mut self,other:Image){
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixels[y as usize][x as usize] += other.pixels[y as usize][x as usize];
            }
        }
    }
}