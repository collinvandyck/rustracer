use super::prelude::*;

pub fn translation(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Matrix {
    let mut dst = identity_matrix();
    dst.set(0, 3, x);
    dst.set(1, 3, y);
    dst.set(2, 3, z);
    dst
}

pub fn scaling(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Matrix {
    let mut dst = identity_matrix();
    dst.set(0, 0, x);
    dst.set(1, 1, y);
    dst.set(2, 2, z);
    dst
}

pub fn rotation_x(rad: impl Into<Num>) -> Matrix {
    let mut dst = identity_matrix();
    let rad = rad.into();
    dst.set(0, 0, 1);
    dst.set(1, 1, Num::cos(rad));
    dst.set(1, 2, -Num::sin(rad));
    dst.set(2, 1, Num::sin(rad));
    dst.set(2, 2, Num::cos(rad));
    dst.set(3, 3, 1);
    dst
}

pub fn rotation_y(rad: impl Into<Num>) -> Matrix {
    let rad = rad.into();
    let mut dst = identity_matrix();
    dst.set(0, 0, Num::cos(rad));
    dst.set(0, 2, Num::sin(rad));
    dst.set(1, 1, 1);
    dst.set(2, 0, -Num::sin(rad));
    dst.set(2, 2, Num::cos(rad));
    dst.set(3, 3, 1);
    dst
}

pub fn rotation_z(rad: impl Into<Num>) -> Matrix {
    let rad = rad.into();
    let mut dst = identity_matrix();
    dst.set(0, 0, -Num::cos(rad));
    dst.set(0, 1, -Num::sin(rad));
    dst.set(1, 0, Num::sin(rad));
    dst.set(1, 1, Num::cos(rad));
    dst.set(2, 2, 1);
    dst.set(3, 3, 1);
    dst
}

#[cfg(test)]
mod tests {
    use std::{
        f32::consts::SQRT_2,
        f64::consts::{FRAC_2_SQRT_PI, FRAC_PI_2, PI},
    };

    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = translation(5, -3, 2);
        let p = point(-3, 4, 5);
        assert_eq!(transform.mul_point(p), point(2, 1, 7));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translation(5, -3, 2);
        let inv = transform.inverse();
        let p = point(-3, 4, 5);
        assert_eq!(inv.mul_point(p), point(-8, 7, 3));
    }

    #[test]
    fn test_translation_does_not_affect_vectors() {
        let transform = translation(5, -3, 2);
        let v = vector(-3, 4, 5);
        assert_eq!(transform.mul_vector(v), v);
    }

    #[test]
    fn test_scaling_matrix_applied_to_a_point() {
        let transform = scaling(2, 3, 4);
        let p = point(-4, 6, 8);
        assert_eq!(transform.mul_point(p), point(-8, 18, 32));
    }

    #[test]
    fn test_scaling_matrix_applied_to_a_vector() {
        let transform = scaling(2, 3, 4);
        let v = vector(-4, 6, 8);
        assert_eq!(transform.mul_vector(v), vector(-8, 18, 32));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = scaling(2, 3, 4);
        let inv = transform.inverse();
        let v = vector(-4, 6, 8);
        assert_eq!(inv.mul_vector(v), vector(-2, 2, 2));
    }

    #[test]
    fn test_reflection_is_scaling_by_a_negative_value() {
        let transform = scaling(-1, 1, 1);
        let p = point(2, 3, 4);
        assert_eq!(transform.mul_point(p), point(-2, 3, 4));
    }

    #[test]
    fn test_rotate_point_around_the_x_axis() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_eq!(
            half_quarter.mul_point(p),
            point(0, SQRT_2 / 2.0, SQRT_2 / 2.0)
        );
        // +z
        assert_eq!(full_quarter.mul_point(p), point(0, 0, 1));
    }

    #[test]
    fn test_rotate_point_around_the_x_axis_inverted() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();
        // -z
        assert_eq!(inv.mul_point(p), point(0, SQRT_2 / 2.0, -SQRT_2 / 2.0));
    }

    #[test]
    fn test_rotate_point_around_the_y_axis() {
        let p = point(0, 0, 1);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_eq!(
            half_quarter.mul_point(p),
            point(SQRT_2 / 2.0, 0, SQRT_2 / 2.0)
        );
        assert_eq!(full_quarter.mul_point(p), point(1, 0, 0));
    }

    #[test]
    fn test_rotate_point_around_the_z_axis() {
        let p = point(0, 1, 0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_eq!(
            half_quarter.mul_point(p),
            point(-SQRT_2 / 2.0, SQRT_2 / 2.0, 0)
        );
        assert_eq!(full_quarter.mul_point(p), point(-1, 0, 0));
    }
}
