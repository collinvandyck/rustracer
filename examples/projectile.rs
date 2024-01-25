#![allow(unused, dead_code)]

use rustracer::prelude::*;
use std::ops::Add;

fn main() {
    let mut p = Projectile {
        pos: point(0, 1, 0),
        vel: vector(1, 1, 0).normalize(),
    };
    let e = Env {
        gravity: vector(0, -0.1, 0),
        wind: vector(-0.01, 0, 0),
    };
    loop {
        println!("p={}", p.pos);
        p = tick(&e, p);
        if p.pos.y() <= 0.0 {
            break;
        }
    }
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
