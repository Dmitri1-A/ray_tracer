use crate::{ray::Ray, vec3::{Dot, Point3, Vec3}};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Dot
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: Dot, ray_tmax: Dot, rec: &mut HitRecord) -> bool;
}