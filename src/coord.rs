pub mod prelude {
    pub use super::*;
}

use std::{fmt, ops};

pub static ORIGIN: Tuple4 = Tuple4([0.0, 0.0, 0.0, 0.0]);

pub fn point(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Point {
    Point::new(x, y, z)
}

pub fn vector(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Vector {
    Vector::new(x, y, z)
}

pub fn tuple(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>, w: impl Into<Num>) -> Tuple4 {
    Tuple4::new(x, y, z, w)
}

pub type Num = f64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    tup: Tuple4,
}

impl PartialEq<Tuple4> for Point {
    fn eq(&self, other: &Tuple4) -> bool {
        &self.tup == other
    }
}

impl PartialEq<Point> for Tuple4 {
    fn eq(&self, other: &Point) -> bool {
        self == &other.tup
    }
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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:.2},{:.2},{:.2})", self.x(), self.y(), self.z())
    }
}

impl From<Point> for Tuple4 {
    fn from(value: Point) -> Self {
        value.tup
    }
}

impl Point {
    pub fn from_tup(mut tup: Tuple4) -> Self {
        assert_eq!(tup.w(), 1.0);
        Point { tup }
    }
    pub fn new(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Self {
        Self {
            tup: Tuple4::new(x, y, z, 1),
        }
    }
    fn add_point(&self, rhs: Point) -> Vector {
        let tup = self.tup + rhs.tup;
        Vector { tup }
    }
    fn sub_point(&self, rhs: Point) -> Vector {
        let tup = self.tup - rhs.tup;
        Vector { tup }
    }
    pub fn add_vector(&self, rhs: Vector) -> Point {
        let tup = self.tup + rhs.tup;
        Point { tup }
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

impl From<Vector> for Tuple4 {
    fn from(value: Vector) -> Self {
        value.tup
    }
}

impl PartialEq<Vector> for Tuple4 {
    fn eq(&self, other: &Vector) -> bool {
        self == &other.tup
    }
}

impl ops::Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        self.add_vector(rhs)
    }
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
    pub fn from_tup(tup: Tuple4) -> Self {
        Self { tup }
    }
    pub fn new(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Self {
        Self {
            tup: Tuple4::new(x, y, z, 0),
        }
    }
    pub fn sub_vector(&self, rhs: Vector) -> Vector {
        let tup = self.tup - rhs.tup;
        Vector { tup }
    }
    pub fn add_vector(&self, rhs: Vector) -> Self {
        let tup = self.tup + rhs.tup;
        Self { tup }
    }
    pub fn mul_scalar(&self, num: impl Into<Num>) -> Self {
        let num = num.into();
        let tup = self.tup.mul_scalar(num);
        Self { tup }
    }
    pub fn magnitude(self) -> Num {
        let sum = self.x().powi(2) + self.y().powi(2) + self.z().powi(2) + self.w().powi(2);
        sum.sqrt()
    }
    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        let tup = self.tup.div_scalar(mag);
        Vector { tup }
    }
    pub fn dot(&self, other: Self) -> Num {
        self.tup.dot(other.tup)
    }
    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
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

impl ops::Mul for Tuple4 {
    type Output = Tuple4;
    fn mul(self, rhs: Self) -> Self::Output {
        Tuple4([
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z(),
            self.w() * rhs.w(),
        ])
    }
}

impl Tuple4 {
    pub fn new(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>, w: impl Into<Num>) -> Self {
        Self([x.into(), y.into(), z.into(), w.into()])
    }
    pub fn x(&self) -> Num {
        self.get(0)
    }
    pub fn y(&self) -> Num {
        self.get(1)
    }
    pub fn z(&self) -> Num {
        self.get(2)
    }
    fn w(&self) -> Num {
        self.get(3)
    }
    pub fn get(&self, idx: usize) -> Num {
        self.0[idx]
    }
    pub fn set_x(&mut self, num: Num) {
        self.set(0, num);
    }
    pub fn set_y(&mut self, num: Num) {
        self.set(1, num);
    }
    pub fn set_z(&mut self, num: Num) {
        self.set(2, num);
    }
    pub fn set_w(&mut self, num: Num) {
        self.set(3, num)
    }
    pub fn set(&mut self, idx: usize, num: Num) {
        self.0[idx] = num;
    }
    pub fn mul_scalar(self, num: impl Into<Num>) -> Self {
        let num = num.into();
        Self([
            self.x() * num,
            self.y() * num,
            self.z() * num,
            self.w() * num,
        ])
    }
    pub fn div_scalar(self, num: impl Into<Num>) -> Self {
        let num = num.into();
        self.mul_scalar(1.0 / num)
    }
    pub fn dot(self, rhs: Self) -> Num {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z() + self.w() + rhs.w()
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

pub fn nums_equal(n1: impl Into<Num>, n2: impl Into<Num>) -> bool {
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

    #[test]
    fn test_multiply_tuple_by_scalar() {
        let t = tuple(1, -2, 3, -4);
        assert_eq!(t.mul_scalar(3.5), tuple(3.5, -7, 10.5, -14));
    }

    #[test]
    fn test_multiply_tuple_by_fraction() {
        let t = tuple(1, -2, 3, -4);
        assert_eq!(t.mul_scalar(0.5), tuple(0.5, -1, 1.5, -2));
    }

    #[test]
    fn test_divide_tuple_by_scalar() {
        let t = tuple(1, -2, 3, -4);
        assert_eq!(t.div_scalar(2), tuple(0.5, -1, 1.5, -2));
    }

    #[test]
    fn test_compute_magnitude_of_vectors() {
        let vecs = [
            (vector(0, 1, 0), 1.0),
            (vector(0, 0, 1), 1.0),
            (vector(1, 2, 3), 14.0_f64.sqrt()),
            (vector(-1, -2, -3), 14.0_f64.sqrt()),
        ];
        for (v, ex) in vecs {
            let mag = v.magnitude();
            assert_eq!(mag, ex, "expected {v:?}.magnitude() == {ex} but was {mag}");
        }
    }

    #[test]
    fn test_normalize_vector() {
        let vecs = [
            (vector(4, 0, 0), vector(1, 0, 0)),
            (vector(1, 2, 3), vector(0.26726, 0.53452, 0.80178)),
        ];
        for (v, ex) in vecs {
            let norm = v.normalize();
            assert_eq!(
                norm, ex,
                "expected {v:?}.normalize() == {ex:?} but was {norm:?}"
            );
        }
    }

    #[test]
    fn test_magnitude_of_normalized_vector() {
        let v = vector(1, 2, 3);
        let n = v.normalize();
        let m = n.magnitude();
        assert_eq!(m, 1.0);
    }

    #[test]
    fn test_dot_product_of_two_tuples() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn test_cross_product_of_two_vectors() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(a.cross(b), vector(-1, 2, -1));
        assert_eq!(b.cross(a), vector(1, -2, 1));
    }
}
