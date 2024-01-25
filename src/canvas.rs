use crate::prelude::*;

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas::new(width, height)
}

pub struct Canvas {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            pixels.push(color(0, 0, 0));
        }
        Self {
            pixels,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_canvas() {
        let c = canvas(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert_eq!(c.pixels.len(), 10 * 20);
        for pixel in c.pixels {
            assert_eq!(pixel, color(0, 0, 0));
        }
    }
}
