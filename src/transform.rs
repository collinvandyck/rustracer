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

#[cfg(test)]
mod tests {
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
}
