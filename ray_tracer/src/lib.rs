use std::{error::Error };

pub mod color;
pub mod hittable;
pub mod sphere;
pub mod vec3;
pub mod ray;
pub mod hittable_list;
pub mod rtweekend;
pub mod interval;
pub mod camera;

use hittable_list::HittableList;
use sphere::Sphere;
use vec3::{Point3};
use camera::Camera;

pub fn render(width: i32) -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, 0.0, -1.0),
        0.5
    )));

    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0
    )));

    let camera = Camera::new(16.0 / 9.0, width);

    camera.render(&world);

    Ok(())
}