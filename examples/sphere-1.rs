#![allow(unused)]
use rustracer::{canvas::canvas, prelude::*};
use std::{fs, process::Command};

fn main() {
    let pixels = 500;
    let width = pixels;
    let height = pixels;
    let red = color(1, 0, 0);
    let mut canvas = canvas(width, height);

    // unit sphere.
    let s = sphere();

    // we start at z=-5, pointed towards the origin
    let r = ray(point(0, 0, -5), vector(0, 0, 1));

    // the wall will be at z=10
    let wall_z = 10.0;

    // we need six units to accommodate the sphere at this distance from it. using 7 gives us
    // margin.
    let wall_size = 7.0;
    println!("wall size: {wall_size}");

    // the size of a pixel in world units.
    let pixel_size = wall_size / (pixels as f64);
    println!("pixel size: {pixel_size}");

    let half = (pixels as f64) / 2.0 * pixel_size;
    println!("half: {half}");

    let ppm = canvas.ppm();
    fs::write("scene.ppm", ppm).expect("could not write scene");

    let wall_ul = point(-half, half, wall_z);
    let wall_ur = point(half, half, wall_z);
    let wall_bl = point(-half, -half, wall_z);
    let wall_br = point(half, -half, wall_z);

    // render the middle line

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
