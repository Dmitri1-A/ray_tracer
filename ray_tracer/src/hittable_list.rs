use crate::{hittable::{HitRecord, Hittable}, ray::Ray, vec3::Dot};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>, // может надо Rc
}

impl HittableList {
    pub fn make_empty() -> Self { Self { objects: vec![] } }

    pub fn new(object: Box<dyn Hittable>) -> Self {
        Self {
            objects: vec![object]
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit<'a>(&self, r: &Ray, ray_tmin: Dot, ray_tmax: Dot, rec: &'a mut HitRecord) -> (bool, &'a mut HitRecord) {
        let temp_rec = rec;
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if let (true, temp_rec) = object.hit(r, ray_tmin, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }

        (hit_anything, temp_rec)
    }
}
