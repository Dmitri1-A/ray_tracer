use crate::vec3::{Dot, Point3, Vec3};

pub struct Ray<'a, 'b> {
    orig: &'a Point3,
    dir: &'b Vec3,
}

impl<'a, 'b> Ray<'a, 'b> {
    pub fn new(origin: &'a Point3, direction: &'b Vec3) -> Self {
        Self { orig: origin, dir: direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: Dot) -> Point3 {
        self.orig + t * self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_at() {
        let d = Vec3::new(1.5, 1.3, 1.1);
        let o = Point3::new(1.0, 2.0, 1.0);

        let point = Ray::new(&d, &o);
    
        assert_eq!("3.5 5.3 3.1", format!("{}", point.at(2.0)));
    }
}