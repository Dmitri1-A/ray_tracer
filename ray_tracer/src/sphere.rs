use crate::{hittable::Hittable, vec3::{dot, Dot, Point3}};

pub struct Sphere {
    center: Point3,
    radius: Dot
}

impl Sphere {
    pub fn new(center: &Point3, radius: Dot) -> Self {
        Self {
            center: Point3 {
                e: center.e.clone()
            },
            radius: (0.0 as Dot).max(radius)
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self,r: &crate::ray::Ray, ray_tmin: Dot, ray_tmax: Dot, rec: &mut crate::hittable::HitRecord) -> bool {
        let oc = &self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false
        }

        let sqrt = discriminant.sqrt();
        let root = (h - sqrt) / a;

        if root <= ray_tmin || ray_tmax <= root {
            let root = (h + sqrt) / a;

            if root <= ray_tmin || ray_tmax <= root {
                return false
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (&rec.p - &self.center) / self.radius;

        return true
    }
}
