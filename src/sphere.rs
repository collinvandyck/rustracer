use std::ops::Sub;

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
        let sphere_to_ray: Vector = ray.origin().sub(self.origin);
        let a: Num = ray.dir().dot(ray.dir());
        let b = 2.0 * ray.dir().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let disc: Num = (b * b) - (4.0 * a * c);
        if disc < 0.0 {
            return Intersection::None;
        }
        let t1 = (-b - f64::sqrt(disc)) / (2.0 * a);
        let t2 = (-b + f64::sqrt(disc)) / (2.0 * a);
        Intersection::Double(t1, t2)
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

    #[test]
    fn ray_originates_inside_sphere() {
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Intersection::Double(-1.0, 1.0));
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = ray(point(0, 0, 5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs, Intersection::Double(-6.0, -4.0));
    }
}
