use std::{fs, process::Command};

use rustracer::{canvas::canvas, prelude::*};

fn main() {
    let mut canvas = canvas(900, 550);
    let start = point(0, 1, 0);
    let vel = vector(1, 1.8, 0).normalize().mul_scalar(11.25);
    let mut p = Projectile { pos: start, vel };
    let e = Env {
        gravity: vector(0, -0.1, 0),
        wind: vector(-0.01, 0, 0),
    };
    let red = color(1, 0, 0);
    loop {
        canvas.write(p.pos.x() as usize, canvas.height - p.pos.y() as usize, red);
        p = tick(&e, p);
        if p.pos.y() <= 0.0 {
            break;
        }
    }
    let ppm = canvas.ppm();
    fs::write("scene.ppm", ppm).expect("could not write scene");
    Command::new("open")
        .arg("scene.ppm")
        .output()
        .expect("could not open scene");
}

fn tick(env: &Env, proj: Projectile) -> Projectile {
    let pos = proj.pos.add_vector(proj.vel);
    let vel = proj.vel + env.gravity + env.wind;
    Projectile { pos, vel }
}

struct Env {
    gravity: Vector,
    wind: Vector,
}

#[derive(Debug)]
struct Projectile {
    pos: Point,
    vel: Vector,
}
