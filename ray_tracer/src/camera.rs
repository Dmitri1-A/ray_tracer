use crate::{
    color::{write_color, Color},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    rtweekend::INFINITY,
    vec3::{unit_vector, Point3, Vec3},
};

use std::io::{self, Write};

pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width over height
    pub image_width: i32, // Rendered image width in pixel count
    image_height: i32, // Rendered image height
    center: Point3, // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = Point3::new(0.0, 0.0, 0.0);
        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = &viewport_u / image_width as f64;
        let pixel_delta_v = &viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = &center - Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3\n{} {} 255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - j);
            io::stderr().flush().unwrap();

            for i in 0..self.image_width {
                let pixel_center = &self.pixel00_loc + (i as f64 * &self.pixel_delta_u) + (j as f64 * &self.pixel_delta_v);
                let ray_direction = pixel_center - &self.center;
                let r = Ray::new(&self.center, &ray_direction);

                let pixel_color = Self::ray_color(&r, world);

                write_color(&pixel_color);
            }
        }

        eprintln!("Done.");
    }

    fn ray_color(r: &Ray, world: &HittableList) -> Color {
        let rec = &mut HitRecord::new();

        if world.hit(r, &Interval::new(0.0, INFINITY), rec) {
            return 0.5 * (&rec.normal + Color::new(1.0, 1.0, 1.0))
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
