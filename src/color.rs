#![allow(dead_code, unused)]

use std::ops;

pub use crate::prelude::*;

pub mod prelude {
    pub use super::*;
}

pub fn color(r: impl Into<Num>, g: impl Into<Num>, b: impl Into<Num>) -> Color {
    Color::new(r, g, b)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    tup: Tuple4,
}

impl ops::Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            tup: self.tup + rhs.tup,
        }
    }
}

impl ops::Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            tup: self.tup - rhs.tup,
        }
    }
}

impl Color {
    pub fn new(r: impl Into<Num>, g: impl Into<Num>, b: impl Into<Num>) -> Self {
        Self {
            tup: Tuple4::new(r, g, b, 0),
        }
    }
    pub fn r(&self) -> Num {
        self.tup.x()
    }
    pub fn g(&self) -> Num {
        self.tup.y()
    }
    pub fn b(&self) -> Num {
        self.tup.z()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_are_tuples() {
        let c = color(-0.5, 0.4, 1.7);
        assert_eq!(c.r(), -0.5);
        assert_eq!(c.g(), 0.4);
        assert_eq!(c.b(), 1.7);
    }

    #[test]
    fn test_add_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_sub_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));
    }
}
