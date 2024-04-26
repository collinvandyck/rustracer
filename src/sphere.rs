use super::prelude::*;

pub fn sphere() -> Sphere {
    Sphere::new(point(0, 0, 0), 1)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    origin: Point,
    radius: Num,
}

impl Sphere {
    pub fn new(origin: Point, radius: impl Into<Num>) -> Self {
        Self {
            origin,
            radius: radius.into(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Intersection {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Intersection::Double(4.0, 6.0));
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = ray(point(0, 1, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Intersection::Double(5.0, 5.0));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = ray(point(0, 2, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Intersection::None);
    }
}
