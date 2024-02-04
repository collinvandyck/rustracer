use std::usize;

use once_cell::sync::Lazy;

use super::prelude::*;

pub static IDENTITY: Lazy<Matrix> = Lazy::new(|| {
    let mut m = Matrix::Matrix4([0.0; 16]);
    for i in 0..4 {
        m.set(i, i, 1.0);
    }
    m
});

pub fn identity_matrix() -> Matrix {
    *IDENTITY
}

#[derive(Debug, Clone, Copy)]
pub enum Matrix {
    Matrix4([Num; 16]),
    Matrix3([Num; 9]),
    Matrix2([Num; 4]),
}

impl Matrix {
    fn new(vals: impl IntoIterator<Item = Num>) -> Self {
        let vals = vals.into_iter().collect::<Vec<_>>();
        let dim = (vals.len() as f64).sqrt() as usize;
        assert_eq!(dim * dim, vals.len());
        match dim {
            4 => {
                let mut arr = [0.0; 16];
                arr.copy_from_slice(&vals);
                Matrix::Matrix4(arr)
            }
            3 => {
                let mut arr = [0.0; 9];
                arr.copy_from_slice(&vals);
                Matrix::Matrix3(arr)
            }
            2 => {
                let mut arr = [0.0; 4];
                arr.copy_from_slice(&vals);
                Matrix::Matrix2(arr)
            }
            _ => panic!("invalid dim: {dim}"),
        }
    }

    fn mul_tuple(&self, tup: Tuple4) -> Tuple4 {
        assert!(
            matches!(self, Matrix::Matrix4(_)),
            "invalid tuple for matrix"
        );
        let mut dst = tup.clone();
        for row in 0..self.rows() {
            dst.set(
                row,
                (0..self.dim()).map(|i| self.get(row, i) * tup.get(i)).sum(),
            );
        }
        dst
    }

    fn mul_matrix(&self, other: Self) -> Self {
        assert!(
            self.same_variant(other),
            "matrices are of different dimensions"
        );
        let mut dst = *self;
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let val = (0..self.dim())
                    .map(|i| self.get(row, i) * other.get(i, col))
                    .sum();
                dst.set(row, col, val);
            }
        }
        return dst;
    }

    fn get(&self, row: usize, col: usize) -> Num {
        let idx = self.idx(row, col);
        match self {
            Matrix::Matrix4(vs) => vs[idx],
            Matrix::Matrix3(vs) => vs[idx],
            Matrix::Matrix2(vs) => vs[idx],
        }
    }

    fn set(&mut self, row: usize, col: usize, val: Num) {
        let idx = self.idx(row, col);
        match self {
            Matrix::Matrix4(vs) => vs[idx] = val,
            Matrix::Matrix3(vs) => vs[idx] = val,
            Matrix::Matrix2(vs) => vs[idx] = val,
        }
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        match self {
            Matrix::Matrix4(_) => row * 4 + col,
            Matrix::Matrix3(_) => row * 3 + col,
            Matrix::Matrix2(_) => row * 2 + col,
        }
    }

    fn rows(&self) -> usize {
        self.dim()
    }

    fn cols(&self) -> usize {
        self.dim()
    }

    fn dim(&self) -> usize {
        match self {
            Matrix::Matrix4(_) => 4,
            Matrix::Matrix3(_) => 3,
            Matrix::Matrix2(_) => 2,
        }
    }

    fn same_variant(&self, other: Self) -> bool {
        match (self, other) {
            (Matrix::Matrix4(_), Matrix::Matrix4(_)) => true,
            (Matrix::Matrix3(_), Matrix::Matrix3(_)) => true,
            (Matrix::Matrix2(_), Matrix::Matrix2(_)) => true,
            _ => false,
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Matrix::Matrix4(vs1), Matrix::Matrix4(vs2)) => {
                for idx in 0..vs1.len() {
                    if !nums_equal(vs1[idx], vs2[idx]) {
                        return false;
                    }
                }
                true
            }
            (Matrix::Matrix3(vs1), Matrix::Matrix3(vs2)) => {
                for idx in 0..vs1.len() {
                    if !nums_equal(vs1[idx], vs2[idx]) {
                        return false;
                    }
                }
                true
            }
            (Matrix::Matrix2(vs1), Matrix::Matrix2(vs2)) => {
                for idx in 0..vs1.len() {
                    if !nums_equal(vs1[idx], vs2[idx]) {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use std::result::Result as StdResult;

    macro_rules! matrix {
        ($s:expr) => {
            matrix_from_spec($s).unwrap()
        };
    }

    #[test]
    fn test_constructing_and_inspecting() {
        let m = matrix!(
            "
            | 1    | 2    | 3    | 4    |
            | 5.5  | 6.5  | 7.5  | 8.5  |
            | 9    | 10   | 11   | 12   |
            | 13.5 | 14.5 | 15.5 | 16.5 | "
        );
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 3), 4.0);
        assert_eq!(m.get(1, 0), 5.5);
        assert_eq!(m.get(1, 2), 7.5);
        assert_eq!(m.get(2, 2), 11.0);
        assert_eq!(m.get(3, 0), 13.5);
        assert_eq!(m.get(3, 2), 15.5);
    }

    #[test]
    fn test_2x2_matrix_representable() {
        let m = matrix!(
            "
            | -3 | 5  |
            | 1  | -2 | "
        );
        assert_eq!(m.get(0, 0), -3.0);
        assert_eq!(m.get(0, 1), 5.0);
        assert_eq!(m.get(1, 0), 1.0);
        assert_eq!(m.get(1, 1), -2.0);
    }

    #[test]
    fn test_3x3_matrix_representable() {
        let m = matrix!(
            "
            | -3 | 5  | 0  |
            | 1  | -2 | -7 |
            | 0  | 1  | 1  | "
        );
        assert_eq!(m.get(0, 0), -3.0);
        assert_eq!(m.get(1, 1), -2.0);
        assert_eq!(m.get(2, 2), 1.0);
    }

    #[test]
    fn test_matrix_equality_with_identical_matrices() {
        let ma = matrix!(
            "
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 | "
        );
        let mb = matrix!(
            "
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 | "
        );
        assert_eq!(ma, mb);
    }

    #[test]
    fn test_matrix_equality_with_different_matrices() {
        let ma = matrix!(
            "
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 | "
        );
        let mb = matrix!(
            "
            | 2 | 3 | 4 | 5 |
            | 6 | 7 | 8 | 9 |
            | 8 | 7 | 6 | 5 |
            | 4 | 3 | 2 | 1 |"
        );
        assert_ne!(ma, mb);
    }

    #[test]
    fn test_multiplying_two_matrixes() {
        let ma = matrix!(
            "
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 | "
        );
        let mb = matrix!(
            "
            | -2 | 1  | 2  | 3  |
            | 3  | 2  | 1  | -1 |
            | 4  | 3  | 6  | 5  |
            | 1  | 2  | 7  | 8  | "
        );
        assert_eq!(
            ma.mul_matrix(mb),
            matrix!(
                "
            | 20  | 22  | 50  | 48  |
            | 44  | 54  | 114 | 108 |
            | 40  | 58  | 110 | 102 |
            | 16  | 26  | 46  | 42  | "
            )
        );
    }

    #[test]
    fn test_multiply_matrix_by_a_tuple() {
        let ma = matrix!(
            "
            | 1 | 2 | 3 | 4 |
            | 2 | 4 | 4 | 2 |
            | 8 | 6 | 4 | 1 |
            | 0 | 0 | 0 | 1 | "
        );
        let b = tuple(1, 2, 3, 1);
        assert_eq!(ma.mul_tuple(b), tuple(18, 24, 33, 1));
    }

    #[test]
    fn test_multiplying_matrix_by_identity() {
        let m = matrix!(
            "
            | 0  | 1  | 2  | 4  |
            | 1  | 2  | 4  | 8  |
            | 2  | 4  | 8  | 16 |
            | 4  | 8  | 16 | 32 | "
        );
        assert_eq!(m.mul_matrix(identity_matrix()), m);
    }

    fn matrix_from_spec(spec: &str) -> anyhow::Result<Matrix> {
        let vals = spec
            .split('|')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<Num>())
            .collect::<StdResult<Vec<_>, _>>()
            .context("failed to parse nums")?;
        Ok(Matrix::new(vals))
    }
}
