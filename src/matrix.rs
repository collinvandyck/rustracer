use std::usize;

use super::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix {
    vals: Storage,
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            vals: self.vals * rhs.vals,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Storage {
    Matrix4([Num; 16]),
    Matrix3([Num; 9]),
    Matrix2([Num; 4]),
}

impl std::ops::Mul for Storage {
    type Output = Storage;
    fn mul(self, rhs: Self) -> Self::Output {
        self
    }
}

impl Storage {
    fn new(vals: impl IntoIterator<Item = Num>) -> Self {
        let vals = vals.into_iter().collect::<Vec<_>>();
        let dim = (vals.len() as f64).sqrt() as usize;
        assert_eq!(dim * dim, vals.len());
        match dim {
            4 => {
                let mut arr = [0.0; 16];
                arr.copy_from_slice(&vals);
                Storage::Matrix4(arr)
            }
            3 => {
                let mut arr = [0.0; 9];
                arr.copy_from_slice(&vals);
                Storage::Matrix3(arr)
            }
            2 => {
                let mut arr = [0.0; 4];
                arr.copy_from_slice(&vals);
                Storage::Matrix2(arr)
            }
            _ => panic!("invalid dim: {dim}"),
        }
    }
    fn get(&self, row: usize, col: usize) -> Num {
        let idx = self.idx(row, col);
        match self {
            Storage::Matrix4(vs) => vs[idx],
            Storage::Matrix3(vs) => vs[idx],
            Storage::Matrix2(vs) => vs[idx],
        }
    }
    fn set(&mut self, row: usize, col: usize, val: Num) {
        let idx = self.idx(row, col);
        match self {
            Storage::Matrix4(vs) => vs[idx] = val,
            Storage::Matrix3(vs) => vs[idx] = val,
            Storage::Matrix2(vs) => vs[idx] = val,
        }
    }
    fn idx(&self, row: usize, col: usize) -> usize {
        match self {
            Storage::Matrix4(_) => row * 4 + col,
            Storage::Matrix3(_) => row * 3 + col,
            Storage::Matrix2(_) => row * 2 + col,
        }
    }
}

impl PartialEq for Storage {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Storage::Matrix4(vs1), Storage::Matrix4(vs2)) => {
                for idx in 0..vs1.len() {
                    if !nums_equal(vs1[idx], vs2[idx]) {
                        return false;
                    }
                }
                true
            }
            (Storage::Matrix3(vs1), Storage::Matrix3(vs2)) => {
                for idx in 0..vs1.len() {
                    if !nums_equal(vs1[idx], vs2[idx]) {
                        return false;
                    }
                }
                true
            }
            (Storage::Matrix2(vs1), Storage::Matrix2(vs2)) => {
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

impl Matrix {
    fn new(vals: impl IntoIterator<Item = Num>) -> Self {
        let vals = Storage::new(vals);
        Self { vals }
    }

    pub fn set(&mut self, row: usize, col: usize, val: Num) {
        self.vals.set(row, col, val);
    }

    pub fn get(&self, row: usize, col: usize) -> Num {
        self.vals.get(row, col)
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
            ma * mb,
            matrix!(
                "
            | 20  | 22  | 50  | 48  |
            | 44  | 54  | 114 | 108 |
            | 40  | 58  | 110 | 102 |
            | 16  | 26  | 46  | 42  | "
            )
        );
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
