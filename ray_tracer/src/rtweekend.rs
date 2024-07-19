use crate::vec3::Dot;

pub const INFINITY: Dot = Dot::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

#[inline]
pub fn degress_to_radians(degress: f64) -> f64 {
    degress * PI / 180.0
}
