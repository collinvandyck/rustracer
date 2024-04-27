use std::ops::Sub;

use super::prelude::*;

pub fn sphere() -> Sphere {
    Sphere::new(point(0, 0, 0), 1)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    origin: Point,
    radius: Num,
    tf: Matrix,
}

impl Sphere {
    pub fn new(origin: Point, radius: impl Into<Num>) -> Self {
        Self {
            origin,
            radius: radius.into(),
            tf: identity(),
        }
    }

    pub fn with_transform(mut self, tf: Matrix) -> Self {
        self.tf = tf;
        self
    }

    pub fn set_transform(&mut self, tf: Matrix) {
        self.tf = tf;
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let ray = ray.transform(self.tf.inverse());
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

    pub fn normal_at(&self, p: Point) -> Vector {
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

    #[test]
    fn sphere_default_transformation() {
        let s = sphere();
        assert_eq!(s.tf, identity());
    }

    #[test]
    fn change_a_sphere_transformation() {
        let mut s = sphere();
        let t = translation(2, 3, 4);
        s.set_transform(t);
        assert_eq!(s.tf, t);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = sphere();
        s.set_transform(scaling(2, 2, 2));
        let xs = s.intersect(r);
        assert_eq!(xs, intersections([intersection(3, s), intersection(7, s),]));
    }

    #[test]
    fn intersection_translated_sphere_with_ray() {
        let r = ray(point(0, 0, -5), vector(0, 0, 1));
        let mut s = sphere();
        s.set_transform(translation(5, 0, 0));
        let xs = s.intersect(r);
        assert_eq!(xs, Intersections::default());
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = sphere();
        let n = s.normal_at(point(1, 0, 0));
        assert_eq!(n, vector(1, 0, 0));
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = sphere();
        let n = s.normal_at(point(0, 1, 0));
        assert_eq!(n, vector(0, 1, 0));
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = sphere();
        let n = s.normal_at(point(0, 0, 1));
        assert_eq!(n, vector(0, 0, 1));
    }

    #[test]
    fn normal_on_a_sphere_at_a_nonaxial_point() {
        let s = sphere();
        let n = s.normal_at(point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(
            n,
            vector(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            )
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = sphere();
        let n = s.normal_at(point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(
            n,
            vector(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            )
        );
        assert_eq!(
            n.normalize(),
            vector(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            )
        );
    }
}
