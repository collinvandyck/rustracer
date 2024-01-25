use std::io;

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
    pub fn write(&mut self, x: usize, y: usize, color: Color) {
        let idx = self.idx(x, y);
        let pixel = self.pixels.get_mut(idx).expect("invalid coords");
        *pixel = color;
    }
    pub fn at(&self, x: usize, y: usize) -> Color {
        let idx = self.idx(x, y);
        *self.pixels.get(idx).expect("invalid coords")
    }
    pub fn ppm(&self) -> Vec<u8> {
        use io::Write;
        let mut buf: Vec<u8> = vec![];
        writeln!(buf, "P3");
        writeln!(buf, "{} {}", self.width, self.height);
        write!(buf, "255");
        buf
    }
    fn idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
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

    #[test]
    fn test_writing_pixels_to_canvas() {
        let mut c = canvas(10, 20);
        let red = color(1, 0, 0);
        c.write(2, 3, red);
        assert_eq!(c.at(2, 3), red);
    }

    #[test]
    fn test_constructing_ppm_header() {
        let c = canvas(5, 3);
        let ppm = c.ppm();
        let ppm = String::from_utf8(ppm).unwrap();
        let ppm = ppm.split('\n').collect::<Vec<_>>();
        assert_eq!(ppm.len(), 3);
        assert_eq!(ppm[0], "P3");
        assert_eq!(ppm[1], "5 3");
        assert_eq!(ppm[2], "255");
    }
}
