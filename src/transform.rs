use super::prelude::*;

pub fn translation(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Matrix {
    let mut dst = identity_matrix();
    dst.set(0, 3, x.into());
    dst.set(1, 3, y.into());
    dst.set(2, 3, z.into());
    dst
}

pub fn scaling(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Matrix {
    let mut dst = identity_matrix();
    dst.set(0, 0, x.into());
    dst.set(1, 1, y.into());
    dst.set(2, 2, z.into());
    dst
}

pub fn rotation_x(x: impl Into<Num>) -> Matrix {
    let mut dst = identity_matrix();
    dst
}

pub fn rotation_y(x: impl Into<Num>) -> Matrix {
    let mut dst = identity_matrix();
    dst
}

pub fn rotation_z(x: impl Into<Num>) -> Matrix {
    let mut dst = identity_matrix();
    dst
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

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
        assert_eq!(half_quarter.mul_point(p), point(0, FRAC_2_SQRT_PI, z))
    }
}
