use std::io;

use crate::prelude::*;

pub fn canvas(width: usize, height: usize) -> Canvas {
    Canvas::new(width, height)
}

pub struct Canvas {
    pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
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
        let max_color = 255;
        let max_line = 70;
        writeln!(buf, "P3");
        writeln!(buf, "{} {}", self.width, self.height);
        writeln!(buf, "{max_color}");
        for row in self.pixels.chunks(self.width).map(|row| {
            row.iter()
                .flat_map(|c| [c.r(), c.g(), c.b()])
                .map(|c| (c * max_color as Num).round() as usize)
                .map(|v| v.clamp(0, max_color))
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
        }) {
            let mut line = String::with_capacity(max_line);
            for num in row {
                if !line.is_empty() && line.len() + 1 + num.len() > max_line {
                    writeln!(buf, "{line}");
                    line.clear();
                }
                if !line.is_empty() {
                    line.push_str(" ");
                }
                line.push_str(&num);
            }
            if !line.is_empty() {
                writeln!(buf, "{line}");
            }
        }
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
        assert_eq!(ppm[0], "P3");
        assert_eq!(ppm[1], "5 3");
        assert_eq!(ppm[2], "255");
    }

    #[test]
    fn test_constructing_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = color(1.5, 0, 0);
        let c2 = color(0, 0.5, 0);
        let c3 = color(-0.5, 0, 1);
        c.write(0, 0, c1);
        c.write(2, 1, c2);
        c.write(4, 2, c3);
        let ppm = c.ppm();
        let ppm = String::from_utf8(ppm).unwrap();
        let ppm = ppm.split('\n').collect::<Vec<_>>();
        assert_eq!(ppm[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(ppm[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(ppm[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn test_splitting_long_lines_in_ppm() {
        let mut c = canvas(10, 2);
        for pixel in c.pixels.iter_mut() {
            *pixel = color(1, 0.8, 0.6);
        }
        let ppm = c.ppm();
        let ppm = String::from_utf8(ppm).unwrap();
        let ppm = ppm.split('\n').collect::<Vec<_>>();
        assert_eq!(
            ppm[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            ppm[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            ppm[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            ppm[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn test_ppm_files_terminated_by_newline() {
        let c = canvas(5, 3);
        let ppm = c.ppm();
        let ppm = String::from_utf8(ppm).unwrap();
        assert!(ppm.ends_with('\n'));
    }
}
