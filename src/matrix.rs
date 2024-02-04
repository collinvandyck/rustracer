use std::usize;

use super::prelude::*;

#[derive(Debug)]
pub struct Matrix {
    vals: Vec<Num>,
    dim: usize,
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.dim != other.dim {
            return false;
        }
        if self.vals.len() != other.vals.len() {
            return false;
        }
        for idx in 0..self.vals.len() {
            if !nums_equal(self.vals[idx], other.vals[idx]) {
                return false;
            }
        }
        true
    }
}

impl Matrix {
    fn new(vals: Vec<Num>) -> Self {
        let dim = (vals.len() as f64).sqrt() as usize;
        assert_eq!(dim * dim, vals.len());
        Self { vals, dim }
    }

    pub fn set(&mut self, row: usize, col: usize, val: Num) {
        let idx = self.idx(row, col);
        self.vals[idx] = val;
    }

    pub fn get(&self, row: usize, col: usize) -> Num {
        self.vals[self.idx(row, col)]
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.dim + col
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
