use super::prelude::*;

pub fn translation(x: impl Into<Num>, y: impl Into<Num>, z: impl Into<Num>) -> Matrix {
    identity_matrix()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = translation(5, -3, 2);
        let p = point(-3, 4, 5);
        assert_eq!(transform.mul_tuple(p), point(2, 1, 7));
    }
}
