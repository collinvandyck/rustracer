use super::prelude::*;
use once_cell::sync::Lazy;
use std::usize;

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

    pub fn mul_tuple(&self, tup: impl Into<Tuple4>) -> Tuple4 {
        let tup = tup.into();
        assert_eq!(self.dim(), 4, "cannot multiply");
        let mut dst = tup.clone();
        for row in 0..self.rows() {
            dst.set(
                row,
                (0..self.dim()).map(|i| self.get(row, i) * tup.get(i)).sum(),
            );
        }
        dst
    }

    pub fn mul_matrix(&self, other: Self) -> Self {
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

    fn transpose(&self) -> Self {
        let mut dst = *self;
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                dst.set(col, row, self.get(row, col));
            }
        }
        dst
    }

    fn determinant(&self) -> Num {
        match self {
            Matrix::Matrix2(_) => self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0),
            _ => (0..self.cols())
                .map(|i| {
                    let cofactor = self.cofactor(0, i);
                    self.get(0, i) * cofactor
                })
                .sum(),
        }
    }

    fn minor(&self, row: usize, col: usize) -> Num {
        let sub = self.submatrix(row, col);
        sub.determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> Num {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 1 {
            -minor
        } else {
            minor
        }
    }

    fn submatrix(&self, del_row: usize, del_col: usize) -> Self {
        let mut dst = match self {
            Matrix::Matrix4(vs) => Matrix::Matrix3([0.0; 9]),
            Matrix::Matrix3(vs) => Matrix::Matrix2([0.0; 4]),
            Matrix::Matrix2(vs) => panic!("submatrix of 2x2 not allowed"),
        };
        for (dr, row) in (0..self.rows()).filter(|i| i != &del_row).enumerate() {
            for (dc, col) in (0..self.cols()).filter(|i| i != &del_col).enumerate() {
                dst.set(dr, dc, self.get(row, col));
            }
        }
        dst
    }

    pub fn inverse(&self) -> Self {
        if !self.invertible() {
            panic!("matrix is not invertible");
        }
        let d = self.determinant();
        let mut dst = *self;
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let c = self.cofactor(row, col);
                dst.set(col, row, c / d);
            }
        }
        dst
    }

    fn invertible(&self) -> bool {
        self.determinant() != 0.0
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
        row * self.dim() + col
    }

    const fn rows(&self) -> usize {
        self.dim()
    }

    const fn cols(&self) -> usize {
        self.dim()
    }

    const fn dim(&self) -> usize {
        match self {
            Matrix::Matrix4(_) => 4,
            Matrix::Matrix3(_) => 3,
            Matrix::Matrix2(_) => 2,
        }
    }

    fn same_variant(&self, other: Self) -> bool {
        self.dim() == other.dim()
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

    #[test]
    fn test_transpose_a_matrix() {
        let m = matrix!(
            "
            | 0 | 9 | 3 | 0 |
            | 9 | 8 | 0 | 8 |
            | 1 | 8 | 5 | 3 |
            | 0 | 0 | 5 | 8 | "
        );
        assert_eq!(
            m.transpose(),
            matrix!(
                "
            | 0 | 9 | 1 | 0 |
            | 9 | 8 | 8 | 0 |
            | 3 | 0 | 5 | 5 |
            | 0 | 8 | 3 | 8 | "
            )
        );
    }

    #[test]
    fn test_transpose_identity_matrix() {
        assert_eq!(identity_matrix().transpose(), identity_matrix());
    }

    #[test]
    fn test_determinant_of_2x2_matrix() {
        let m = matrix!(
            "
            | 1  | 5  |
            | -3 | 2  | "
        );
        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn test_submatrix_of_3x3_matrix() {
        let m = matrix!(
            "
            | 1  | 5  | 0  |
            | -3 | 2  | 7  |
            | 0  | 6  | -3 | "
        );
        assert_eq!(
            m.submatrix(0, 2),
            matrix!(
                "
            | -3 | 2 |
            |  0 | 6 |"
            )
        );
    }

    #[test]
    fn test_submatrix_of_4x4_matrix() {
        let m = matrix!(
            "
            | -6 | 1  | 1  | 6  |
            | -8 | 5  | 8  | 6  |
            | -1 | 0  | 8  | 2  |
            | -7 | 1  | -1 | 1  | "
        );
        assert_eq!(
            m.submatrix(2, 1),
            matrix!(
                "
            | -6 | 1  | 6  |
            | -8 | 8  | 6  |
            | -7 | -1 | 1  | "
            )
        );
    }

    #[test]
    fn test_calculating_minor_of_3x3_matrix() {
        let a = matrix!(
            "
            | 3  | 5  | 0  |
            | 2  | -1 | -7 |
            | 6  | -1 | 5  | "
        );
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn test_calculating_cofactor_of_3x3_matrix() {
        let a = matrix!(
            "
            | 3  | 5  | 0  |
            | 2  | -1 | -7 |
            | 6  | -1 | 5  | "
        );
        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_determinant_of_3x3_matrix() {
        let a = matrix!(
            "
            | 1  | 2  | 6  |
            | -5 | 8  | -4 |
            | 2  | 6  | 4  | "
        );
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn test_determinant_of_4x4_matrix() {
        let a = matrix!(
            "
            | -2 | -8 | 3  | 5  |
            | -3 | 1  | 7  | 3  |
            | 1  | 2  | -9 | 6  |
            | -6 | 7  | 7  | -9 | "
        );
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn test_an_invertible_matrix_for_invertability() {
        let a = matrix!(
            "
            | 6  | 4  | 4  | 4  |
            | 5  | 5  | 7  | 6  |
            | 4  | -9 | 3  | -7 |
            | 9  | 1  | 7  | -6 | "
        );
        assert_eq!(a.determinant(), -2120.0);
        assert!(a.invertible());
    }

    #[test]
    fn test_a_noninvertible_matrix_for_invertability() {
        let a = matrix!(
            "
            | -4 | 2  | -2 | -3 |
            | 9  | 6  | 2  | 6  |
            | 0  | -5 | 1  | -5 |
            | 0  | 0  | 0  | 0  | "
        );
        assert_eq!(a.determinant(), 0.0);
        assert!(!a.invertible());
    }

    #[test]
    fn test_calculate_inverse_of_matrix() {
        let a = matrix!(
            "
            | -5 | 2  | 6  | -8 |
            | 1  | -5 | 1  | 8  |
            | 7  | 7  | -6 | -7 |
            | 1  | -3 | 7  | 4  | "
        );
        let b = a.inverse();
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b.get(3, 2), -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b.get(2, 3), 105.0 / 532.0);
        assert_eq!(
            b,
            matrix!(
                "
            | 0.21805  | 0.45113  | 0.2406   | -0.04511 |
            | -0.80827 | -1.45677 | -0.44361 | 0.52068  |
            | -0.07895 | -0.22368 | -0.05263 | 0.19737  |
            | -0.52256 | -0.81391 | -0.30075 | 0.30639  | "
            )
        );
    }

    #[test]
    fn test_calculating_inverse_of_another_matrix() {
        let a = matrix!(
            "
            | 8  | -5 | 9  | 2  |
            | 7  | 5  | 6  | 1  |
            | -6 | 0  | 9  | 6  |
            | -3 | 0  | -9 | -4 | "
        );
        assert_eq!(
            a.inverse(),
            matrix!(
                "
            | -0.15385 | -0.15385 | -0.28205 | -0.53846 |
            | -0.07692 | 0.12308  | 0.02564  | 0.03077  |
            | 0.35897  | 0.35897  | 0.4359   | 0.92308  |
            | -0.69231 | -0.69231 | -0.76923 | -1.92308 | "
            )
        );
    }

    #[test]
    fn test_calculating_inverse_of_a_third_matrix() {
        let a = matrix!(
            "
            | 9  | 3  | 0  | 9  |
            | -5 | -2 | -6 | -3 |
            | -4 | 9  | 6  | 4  |
            | -7 | 6  | 6  | 2  | "
        );
        assert_eq!(
            a.inverse(),
            matrix!(
                "
            | -0.04074 | -0.07778 | 0.14444  | -0.22222 |
            | -0.07778 | 0.03333  | 0.36667  | -0.33333 |
            | -0.02901 | -0.1463  | -0.10926 | 0.12963  |
            | 0.17778  | 0.06667  | -0.26667 | 0.33333  | "
            )
        );
    }

    #[test]
    fn test_multiplying_a_product_by_its_inverse() {
        let a = matrix!(
            "
            | 3  | -9 | 7  | 3  |
            | 3  | -8 | 2  | -9 |
            | -4 | 4  | 4  | 1  |
            | -6 | 5  | -1 | 1  | "
        );
        let b = matrix!(
            "
            | 8  | 2  | 2  | 2  |
            | 3  | -1 | 7  | 0  |
            | 7  | 0  | 5  | 4  |
            | 6  | -2 | 0  | 5  | "
        );
        let c = a.mul_matrix(b);
        assert_eq!(c.mul_matrix(b.inverse()), a);
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
