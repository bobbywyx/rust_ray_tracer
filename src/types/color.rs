use super::{vec3::Vec3 as color, interval::Interval};

pub struct RGB{
    pub r:u8,
    pub g:u8,
    pub b:u8,
}


pub fn write_color(pixel_color:color,sample_per_pixel:i32){
    let mut r = pixel_color.0;
    let mut g = pixel_color.1;
    let mut b = pixel_color.2;

    // Divide the color by the number of samples.
    let scale = 1.0 / sample_per_pixel as f64;
    r*=scale;
    g*=scale;
    b*=scale;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.0,0.999);

    let ir = (256.0 * intensity.clamp(r)) as i32;
    let ig = (256.0 * intensity.clamp(g)) as i32;
    let ib = (256.0 * intensity.clamp(b)) as i32;
    print!("{} {} {}\n",ir,ig,ib);
}

fn linear_to_gamma(linear_comp:f64) -> f64{
    linear_comp.sqrt()
}

