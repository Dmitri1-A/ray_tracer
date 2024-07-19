use std::{fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub}};

pub type Dot = f64;

pub type Point3 = Vec3;

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub e: [Dot; 3],
}

impl Vec3 {
    pub fn empty() -> Self {
        Self {
            e: [0.0, 0.0, 0.0]
        }
    }

    pub fn new(e0: Dot, e1: Dot, e2: Dot) -> Self {
        Self {
            e: [e0, e1, e2]
        }
    }

    pub fn x(&self) -> Dot { self[0] }
    pub fn y(&self) -> Dot { self[1] }
    pub fn z(&self) -> Dot { self[2] }

    pub fn length(&self) -> Dot {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Dot {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

}

impl Display for Vec3{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { e: [-self[0], -self[1], -self[2]]}
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output { e: [-self[0], -self[1], -self[2]]}
    }
}

impl Index<usize> for Vec3 {
    type Output = Dot;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            e: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2]
            ]
        }
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            e: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2]
            ]
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Self {
            e: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2]
            ]
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            e: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2]
            ]
        }
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            e: [
                self[0] * rhs[0],
                self[1] * rhs[1],
                self[2] * rhs[2]
            ]
        }
    }
}

impl Mul<&Vec3> for Dot {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            e: [
                self * rhs[0],
                self * rhs[1],
                self * rhs[2]
            ]
        }
    }
}

impl Mul<Vec3> for Dot {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            e: [
                self * rhs[0],
                self * rhs[1],
                self * rhs[2]
            ]
        }
    }
}

impl Mul<Dot> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Dot) -> Self::Output {
        Self::Output {
            e: [
                rhs * self[0],
                rhs * self[1],
                rhs * self[2]
            ]
        }
    }
}

impl MulAssign<Dot> for Vec3 {
    fn mul_assign(&mut self, rhs: Dot) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            e: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2]
            ]
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            e: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2]
            ]
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            e: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2]
            ]
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            e: [
                self[0] - rhs[0],
                self[1] - rhs[1],
                self[2] - rhs[2]
            ]
        }
    }
}

impl Div<Dot> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Dot) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl Div<Dot> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Dot) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign<Dot> for Vec3 {
    fn div_assign(&mut self, rhs: Dot) {
        self[0] *= 1.0 as Dot / rhs;
        self[1] *= 1.0 as Dot / rhs;
        self[2] *= 1.0 as Dot / rhs;
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> Dot {
    u[0] * v[0]
    + u[1] * v[1]
    + u[2] * v[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0])
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_empty() {
        let point = Vec3::empty();
        assert_eq!(point.x(), 0.0);
        assert_eq!(point.y(), 0.0);
        assert_eq!(point.z(), 0.0);
    }

    #[test]
    fn vec3_new() {
        let point = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.z(), 3.0);
    }

    #[test]
    fn vec3_negative() {
        let mut point = Vec3::new(1.0, 2.0, 3.0);
        point = -point;

        assert_eq!(-1.0, point[0]);
        assert_eq!(-2.0, point[1]);
        assert_eq!(-3.0, point[2]);
    }

    #[test]
    fn vec3_indexing() {
        let point = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(point[0], 1.0);
        assert_eq!(point[1], 2.0);
        assert_eq!(point[2], 3.0);
    }

    #[test]
    #[should_panic]
    fn vec3_indexing_panic() {
        let point = Vec3::new(1.0, 2.0, 3.0);
        point[3];
    }

    #[test]
    fn vec3_index_mut() {
        let mut point = Vec3::new(1.0, 2.0, 3.0);
        point[0] = 1.56;

        assert_eq!(1.56, point[0])
    }

    #[test]
    fn vec3_add_assign_vec3() {
        let mut point = Vec3::new(1.0, 2.0, 3.0);
        point += &Vec3::new(0.5, 1.5, 2.5);

        assert_eq!(1.5, point[0]);
        assert_eq!(3.5, point[1]);
        assert_eq!(5.5, point[2]);
    }

    #[test]
    fn vec3_add_vec3() {
        let p1 = Vec3::new(1.0, 2.0, 3.0) + &Vec3::new(0.5, 0.3, 0.45);

        assert_eq!(1.5, p1[0]);
        assert_eq!(2.3, p1[1]);
        assert_eq!(3.45, p1[2]);
    }

    #[test]
    fn vec3_mul_assign_dot() {
        let mut point = Vec3::new(1.1, 2.2, 3.3);
        point *= 3.0;

        assert_eq!("3.3", format!("{:.1}", point[0]));
        assert_eq!("6.6", format!("{:.1}", point[1]));
        assert_eq!("9.9", format!("{:.1}", point[2]));
    }

    #[test]
    fn vec3_mul_dot() {
        let point = Vec3::new(1.1, 2.2, 3.3) * 2.0;

        assert_eq!("2.2", format!("{:.1}", point[0]));
        assert_eq!("4.4", format!("{:.1}", point[1]));
        assert_eq!("6.6", format!("{:.1}", point[2]));
    }

    #[test]
    fn vec3_mul_vec3() {
        let point = Vec3::new(1.1, 2.2, 3.3) * &Vec3::new(2.0, 1.5, 1.0);

        assert_eq!("2.2", format!("{:.1}", point[0]));
        assert_eq!("3.3", format!("{:.1}", point[1]));
        assert_eq!("3.3", format!("{:.1}", point[2]));
    }

    #[test]
    fn dot_mul_vec3() {
        let point = 3.0 * &Vec3::new(1.1, 2.2, 3.3);

        assert_eq!("3.3", format!("{:.1}", point[0]));
        assert_eq!("6.6", format!("{:.1}", point[1]));
        assert_eq!("9.9", format!("{:.1}", point[2]));
    }

    #[test]
    fn vec3_div_assign_dot() {
        let mut point = Vec3::new(3.3, 6.6, 9.9);
        point /= 3.0;

        assert_eq!("1.1", format!("{:.1}", point[0]));
        assert_eq!("2.2", format!("{:.1}", point[1]));
        assert_eq!("3.3", format!("{:.1}", point[2]));
    }

    #[test]
    fn vec3_div_dot() {
        let point = Vec3::new(3.3, 6.6, 9.9) / 3.0;

        assert_eq!("1.1", format!("{:.1}", point[0]));
        assert_eq!("2.2", format!("{:.1}", point[1]));
        assert_eq!("3.3", format!("{:.1}", point[2]));
    }

    #[test]
    fn vec3_sub_vec3() {
        let p1 = Vec3::new(1.0, 2.0, 3.0) - &Vec3::new(0.5, 0.3, 0.45);

        assert_eq!(0.5, p1[0]);
        assert_eq!(1.7, p1[1]);
        assert_eq!(2.55, p1[2]);
    }

    #[test]
    fn length() {
        let p = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!("5.385165", format!("{:.6}", p.length()));
    }

    #[test]
    fn length_squared() {
        let p = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!("29.0", format!("{:.1}", p.length_squared()));
    }

    #[test]
    fn fmt_display() {
        let p = Vec3::new(2.3, 3.45, 4.76);

        assert_eq!("2.3 3.45 4.76", format!("{}", p));
    }

    #[test]
    fn dot_vec3_vec3() {
        let u = Vec3::new(2.3, 3.45, 4.76);
        let v = Vec3::new(0.35, 1.33, 2.05);

        assert_eq!("15.151500", format!("{:.6}", dot(&u, &v)));
    }

    #[test]
    fn cross_vec3_vec3() {
        let u = Vec3::new(2.0, 3.0, 4.0);
        let v = Vec3::new(0.0, 1.0, 2.0);

        assert_eq!("2 -4 2", format!("{:.6}", cross(&u, &v)));
    }

    #[test]
    fn unit_vector_vec3() {
        let v = Vec3::new(2.0, 3.0, 4.0);
        let v = unit_vector(&v);

        assert_eq!("0.371391 0.557086 0.742781", format!("{:.6} {:.6} {:.6}", v[0], v[1], v[2]));
    }
}
