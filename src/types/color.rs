use super::{interval::Interval, vec3::Vec3 as color};
use crate::types::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub fn write_color(pixel_color: color, sample_per_pixel: i32, mut f: &File) {
    let mut r = pixel_color.0;
    let mut g = pixel_color.1;
    let mut b = pixel_color.2;

    // Divide the color by the number of samples.
    let scale = 1.0 / sample_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.0, 0.999);

    let ir = (256.0 * intensity.clamp(r)) as i32;
    let ig = (256.0 * intensity.clamp(g)) as i32;
    let ib = (256.0 * intensity.clamp(b)) as i32;
    // print!("{} {} {}\n",ir,ig,ib);
    f.write_fmt(format_args!("{} {} {}\n", ir, ig, ib)).unwrap();
}

fn linear_to_gamma(linear_comp: f64) -> f64 {
    linear_comp.sqrt()
}
