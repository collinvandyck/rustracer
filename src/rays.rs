use super::prelude::*;

pub fn ray(origin: impl Into<Point>, dir: impl Into<Vector>) -> Ray {
    Ray {
        origin: origin.into(),
        dir: dir.into(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Point,
    dir: Vector,
}

impl Ray {
    pub fn position(&self, t: impl Into<Num>) -> Point {
        let dir = self.dir.mul_scalar(t);
        self.origin.add_vector(dir)
    }

    pub fn transform(&self, m: Matrix) -> Self {
        Self {
            origin: m.mul_point(self.origin),
            dir: m.mul_vector(self.dir),
        }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn dir(&self) -> Vector {
        self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_query_array() {
        let origin = point(1, 2, 3);
        let dir = vector(4, 5, 6);
        let r = ray(origin, dir);
        assert_eq!(r.origin, origin);
        assert_eq!(r.dir, dir);
    }

    #[test]
    fn computing_point_from_a_distance() {
        let r = ray(point(2, 3, 4), vector(1, 0, 0));
        assert_eq!(r.position(0), point(2, 3, 4));
        assert_eq!(r.position(1), point(3, 3, 4));
        assert_eq!(r.position(-1), point(1, 3, 4));
        assert_eq!(r.position(2.5), point(4.5, 3, 4));
    }

    #[test]
    fn translating_a_ray() {
        let r = ray(point(1, 2, 3), vector(0, 1, 0));
        let m = translation(3, 4, 5);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, point(4, 6, 8));
        assert_eq!(r2.dir, vector(0, 1, 0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = ray(point(1, 2, 3), vector(0, 1, 0));
        let m = scaling(2, 3, 4);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, point(2, 6, 12));
        assert_eq!(r2.dir, vector(0, 3, 0));
    }
}
