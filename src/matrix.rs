use std::usize;

use super::prelude::*;

pub fn matrix() -> Matrix4 {
    Matrix4::new()
}

pub struct Matrix4 {
    vals: [Num; 16],
}

impl std::ops::Deref for Matrix4 {
    type Target = [Num; 16];
    fn deref(&self) -> &Self::Target {
        &self.vals
    }
}

impl std::ops::DerefMut for Matrix4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vals
    }
}

impl Matrix4 {
    fn new() -> Self {
        Self { vals: [0.0; 16] }
    }

    pub fn set(&mut self, row: usize, col: usize, val: Num) {
        self.vals[self.idx(row, col)] = val;
    }

    pub fn get(&self, row: usize, col: usize) -> Num {
        self.vals[self.idx(row, col)]
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * 4 + col
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use std::result::Result as StdResult;

    macro_rules! matrix4 {
        ($s:expr) => {
            matrix4_from_spec($s).unwrap()
        };
    }

    #[test]
    fn test_constructing_and_inspecting() {
        let m = matrix4!(
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

    fn matrix4_from_spec(spec: &str) -> anyhow::Result<Matrix4> {
        let vals = spec
            .split('|')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<Num>())
            .collect::<StdResult<Vec<_>, _>>()
            .context("failed to parse nums")?;
        let dim = (vals.len() as f64).sqrt() as usize;
        assert_eq!(dim, 4);
        let mut m = Matrix4::new();
        for (idx, val) in vals.into_iter().enumerate() {
            m.vals[idx] = val;
        }
        Ok(m)
    }
}
