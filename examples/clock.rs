use rustracer::{canvas::canvas, prelude::*};
use std::{f64::consts::PI, fs, process::Command};

fn main() {
    let width = 500;
    let height = 500;
    let pixel_size = 10;
    let red = color(1, 0, 0);
    let mut canvas = canvas(width, height);
    let mut write_pixel = |p: Point| {
        for xs in 0..pixel_size {
            for ys in 0..pixel_size {
                let x = p.x() + (xs as f64);
                let y = p.y() + (ys as f64);
                canvas.write(x as usize, y as usize, red);
            }
        }
    };

    let radius = (width as f64) * 3.0 / 8.0;
    let xf = identity()
        .scaling(radius, radius, 1)
        .scaling(1, -1, 1)
        .translation((width as f64) / 2.0, (height as f64) / 2.0, 0);

    // start at noon and rotate around.
    let p = point(0, 1, 0);
    for x in 0..12 {
        let hour = PI / 6.0;
        write_pixel(xf.mul_point(rotation_z((x as f64) * hour).mul_point(p)));
    }

    let ppm = canvas.ppm();
    fs::write("scene.ppm", ppm).expect("could not write scene");
    Command::new("open")
        .arg("scene.ppm")
        .output()
        .expect("could not open scene");
}
