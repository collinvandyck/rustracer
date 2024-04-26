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

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let sphere_to_ray: Vector = ray.origin().sub(self.origin);
        let a: Num = ray.dir().dot(ray.dir());
        let b = 2.0 * ray.dir().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let disc: Num = (b * b) - (4.0 * a * c);
        if disc < 0.0 {
            return Intersections::default();
        }
        let t1 = (-b - f64::sqrt(disc)) / (2.0 * a);
        let t2 = (-b + f64::sqrt(disc)) / (2.0 * a);
        intersections([intersection(t1, *self), intersection(t2, *self)])
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
        assert_eq!(
            xs,
            intersections([intersection(4.0, s), intersection(6.0, s),])
        );
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = ray(point(0, 1, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(
            xs,
            intersections([intersection(5.0, s), intersection(5.0, s),])
        );
    }

    #[test]
    fn ray_misses_sphere() {
        let r = ray(point(0, 2, -5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = ray(point(0, 0, 0), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(
            xs,
            intersections([intersection(-1.0, s), intersection(1.0, s),])
        );
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = ray(point(0, 0, 5), vector(0, 0, 1));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(
            xs,
            intersections([intersection(-6.0, s), intersection(-4.0, s),])
        );
    }
}
