#![allow(dead_code, unused)]

use std::ops;

pub fn point(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Point {
    Point::new(x, y, z)
}

pub fn vector(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Vector {
    Vector::new(x, y, z)
}

pub fn tuple(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>, w: impl Into<Num>) -> Tuple4 {
    Tuple4::new(x, y, z, w)
}

type Num = f64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    tup: Tuple4,
}

impl ops::Sub for Point {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_point(rhs)
    }
}

impl ops::Deref for Point {
    type Target = Tuple4;
    fn deref(&self) -> &Self::Target {
        &self.tup
    }
}

impl ops::DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tup
    }
}

impl Point {
    pub fn new(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Self {
        Self {
            tup: Tuple4::new(x, y, z, 1),
        }
    }
    fn sub_point(&self, rhs: Point) -> Vector {
        let tup = self.tup - rhs.tup;
        Vector { tup }
    }
    pub fn sub_vector(&self, rhs: Vector) -> Point {
        let tup = self.tup - rhs.tup;
        Point { tup }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector {
    tup: Tuple4,
}

impl ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_vector(rhs)
    }
}

impl ops::Deref for Vector {
    type Target = Tuple4;
    fn deref(&self) -> &Self::Target {
        &self.tup
    }
}

impl ops::DerefMut for Vector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tup
    }
}

impl Vector {
    pub fn new(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Self {
        Self {
            tup: Tuple4::new(x, y, z, 0),
        }
    }
    pub fn sub_vector(&self, rhs: Vector) -> Vector {
        let tup = self.tup - rhs.tup;
        Vector { tup }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple4([Num; 4]);

impl ops::Add for Tuple4 {
    type Output = Tuple4;
    fn add(self, rhs: Self) -> Self::Output {
        Tuple4([
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
            self.w() + rhs.w(),
        ])
    }
}

impl ops::Sub for Tuple4 {
    type Output = Tuple4;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple4([
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
            self.w() - rhs.w(),
        ])
    }
}

impl ops::Neg for Tuple4 {
    type Output = Tuple4;
    fn neg(self) -> Self::Output {
        Tuple4([
            -1.0 * self.x(),
            -1.0 * self.y(),
            -1.0 * self.z(),
            -1.0 * self.w(),
        ])
    }
}

impl Tuple4 {
    pub fn new(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>, w: impl Into<Num>) -> Self {
        Self([x.into(), y.into(), z.into(), w.into()])
    }
    pub fn x(&self) -> Num {
        self.0[0]
    }
    pub fn y(&self) -> Num {
        self.0[1]
    }
    pub fn z(&self) -> Num {
        self.0[2]
    }
    pub fn set_x(&mut self, num: Num) {
        self.0[0] = num
    }
    pub fn set_y(&mut self, num: Num) {
        self.0[1] = num
    }
    pub fn set_z(&mut self, num: Num) {
        self.0[2] = num
    }
    fn w(&self) -> Num {
        self.0[3]
    }
    fn set_w(&mut self, num: Num) {
        self.0[3] = num;
    }
    fn is_point(&self) -> bool {
        self.0[3] == 1.0
    }
    fn is_vector(&self) -> bool {
        self.0[3] == 0.0
    }
}

impl PartialEq for Tuple4 {
    fn eq(&self, other: &Self) -> bool {
        nums_equal(self.x(), other.x())
            && nums_equal(self.y(), other.y())
            && nums_equal(self.z(), other.z())
            && nums_equal(self.w(), other.w())
    }
}

fn nums_equal(n1: impl Into<Num>, n2: impl Into<Num>) -> bool {
    const EPSILON: f64 = 0.00001;
    (n1.into() - n2.into()).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    #[test]
    fn test_tuples() {
        let t = tuple(4.3, -4.2, 3.1, 1);
        assert_eq!([t.x(), t.y(), t.z(), t.w()], [4.3, -4.2, 3.1, 1.0]);
        assert!(t.is_point());
        assert!(!t.is_vector());

        let t = tuple(4.3, -4.2, 3.1, 0);
        assert_eq!([t.x(), t.y(), t.z(), t.w()], [4.3, -4.2, 3.1, 0.0]);
        assert!(!t.is_point());
        assert!(t.is_vector());
    }

    #[test]
    fn test_tuple_convenience() {
        let p = point(4, -4, 3);
        assert_eq!(p.deref(), &tuple(4, -4, 3, 1));

        let v = vector(4, -4, 3);
        assert_eq!(v.deref(), &tuple(4, -4, 3, 0));
    }

    #[test]
    fn test_add_tuples() {
        let a1 = tuple(3, -2, 5, 1);
        let a2 = tuple(-2, 3, 1, 0);
        assert_eq!(a1 + a2, tuple(1, 1, 6, 1));
    }

    #[test]
    fn test_subtract_points() {
        let p1 = point(3, 2, 1);
        let p2 = point(5, 6, 7);
        assert_eq!(p1 - p2, vector(-2, -4, -6));
    }

    #[test]
    fn test_subtract_vector_from_point() {
        let p1 = point(3, 2, 1);
        let v1 = vector(5, 6, 7);
        assert_eq!(p1.sub_vector(v1), point(-2, -4, -6));
    }

    #[test]
    fn test_subtract_vector_from_vector() {
        let v1 = vector(3, 2, 1);
        let v2 = vector(5, 6, 7);
        assert_eq!(v1 - v2, vector(-2, -4, -6));
    }

    #[test]
    fn test_subtract_vector_from_zero_vector() {
        let z = vector(0, 0, 0);
        let v = vector(1, -2, 3);
        assert_eq!(v - z, vector(1, -2, 3));
    }

    #[test]
    fn test_negate_tuple() {
        let a = tuple(1, -2, 3, -4);
        assert_eq!(-a, tuple(-1, 2, -3, 4))
    }
}
