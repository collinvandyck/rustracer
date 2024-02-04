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

    #[test]
    fn test_constructing_and_inspecting() {
        let m = matrix();
        let spec =
            "|1|2|3|4| | 5.5| 6.5| 7.5| 8.5| | 9 | 10 | 11 | 12 | | 13.5 | 14.5 | 15.5 | 16.5 |";
    }
}
