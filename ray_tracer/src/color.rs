
use std::io::{self, Write};

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) -> std::io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let r = (r * 255.999) as u8;
    let g = (g * 255.999) as u8;
    let b = (b * 255.999) as u8;

    println!("{r} {g} {b}");

    io::stdout().flush()
}