use crate::{ray::Ray, vec3::{dot, Dot, Point3, Vec3}};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Dot,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            front_face: false,
            normal: Vec3::empty(),
            t: 0.0,
            p: Vec3::empty()
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { (*outward_normal).clone() } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit<'a>(&self, r: &Ray, ray_tmin: Dot, ray_tmax: Dot, rec: &'a mut HitRecord) -> (bool, &'a mut HitRecord);
}