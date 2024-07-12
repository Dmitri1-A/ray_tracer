use std::{error::Error, io::{self, Write}};

pub mod color;
pub mod hittable;
pub mod sphere;
pub mod vec3;
pub mod ray;

use color::{write_color, Color};
use ray::Ray;
use vec3::{dot, unit_vector, Dot, Point3, Vec3};

pub fn render(width: i32) -> Result<(), Box<dyn Error>> {
    // Calculate the image height, and ensure that it's at least 1.
    let aspect_ratio = 16.0 / 9.0;
    let height = (width as f64 / aspect_ratio) as i32;
    let height = if height < 1 { 1 } else { height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f64 / height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = &viewport_u / (width as f64);
    let pixel_delta_v = &viewport_v / (height as f64);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = &camera_center
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

    // Render

    println!("P3");
    println!("{width} {height}");
    println!("255");

    for j in 0..height {
        eprintln!("Scanlines remaining: {}", height - j);
        io::stderr().flush()?;

        for i in 0..width {
            let pixel_center = &pixel00_loc + (i as f64 * &pixel_delta_u) + (j as f64 * &pixel_delta_v);
            let ray_direction = pixel_center - &camera_center;

            let pixel_color = ray_color(&Ray::new(&camera_center, &ray_direction));

            write_color(&pixel_color)?;
        }
    }

    eprintln!("Done.");

    Ok(())
}

fn hit_sphere(center: &Point3, radius: Dot, r: &Ray) -> Dot {
    let oc = center - r.origin();
    let a = r.direction().length_squared();
    let h = dot(r.direction(), &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let n = unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0 , n.z() + 1.0)
    }

    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
