use rustracer::{canvas::canvas, prelude::*};
use std::{fs, process::Command, time::Instant};

fn main() {
    let start = Instant::now();
    let dim = 500;
    let width = dim;
    let height = dim;
    let mut canvas = canvas(width, height);

    let s = sphere();
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / (dim as f64);
    let half = (dim as f64) / 2.0 * pixel_size;
    let color = color(0.5, 0.25, 0);
    let ray_p = point(0, 0, -5);
    for y in 0..dim {
        let wy = half - pixel_size * (y as f64);
        for x in 0..dim {
            let wx = -half + pixel_size * (x as f64);
            let pos = point(wx, wy, wall_z);
            let vec = (pos - ray_p).normalize();
            let r = ray(ray_p, vec);
            if s.intersect(r).hit().is_some() {
                canvas.write(x, y, color);
            }
        }
    }

    let elapsed = start.elapsed() / 1000 * 1000;
    let per_pixel = elapsed / ((width * height) as u32);
    println!("render complete ({elapsed:?}) ({per_pixel:?} / pixel)");
    let ppm = canvas.ppm();
    fs::write("scene.ppm", ppm).expect("could not write scene");
    assert!(Command::new("open")
        .args(["scene.ppm"])
        .output()
        .expect("could not open scene")
        .status
        .success());
    assert!(Command::new("/usr/bin/osascript")
        .args(["-e", "tell application \"Kitty\" to activate"])
        .output()
        .expect("applescript failed")
        .status
        .success());
}
