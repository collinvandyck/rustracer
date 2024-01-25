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

#[derive(Debug, PartialEq)]
pub struct Point {
    tup: Tuple4,
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
}

#[derive(Debug, PartialEq)]
pub struct Vector {
    tup: Tuple4,
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
}

#[derive(Debug, PartialEq)]
pub struct Tuple4([Num; 4]);

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
}
