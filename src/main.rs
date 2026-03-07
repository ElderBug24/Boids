#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod boid;
mod shape;

use boid::{Boid, Vehicle, Food};

use std::time::{Duration, Instant};
use std::thread::sleep;

use ::rand::{Rng, RngExt};
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 800;
const TARGET_FPS: f32 = 60.0;


fn window_conf() -> Conf {
    return Conf {
        window_title: "Boids Evolution Simulation".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        platform: miniquad::conf::Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    };
}

const BACKGROUND_COLOR: Color = Color::from_rgba(45, 41, 61, 255);
const TEXT_COLOR: Color = Color::from_rgba(245, 0, 179, 255);
const DEBUG: bool = false;

#[macroquad::main(window_conf)]
async fn main() {
    let mut rng = ::rand::rng();
    let mut boids: Vec<Boid> = Vec::new();
    let mut food: Vec<(Food, Vec2)> = Vec::new();

    for _ in 0..50 { // 8
        let x = rng.random_range(0.0..WINDOW_WIDTH as f32);
        let y = rng.random_range(0.0..WINDOW_HEIGHT as f32);
        boids.push(Boid::new(vec2(x, y)));
    }

    for _ in 0..80 { // 20
        let x = rng.random_range(0.0..WINDOW_WIDTH as f32);
        let y = rng.random_range(0.0..WINDOW_HEIGHT as f32);
        food.push(({
            match rng.random_bool(0.5) {
                true => Food::Apple,
                false => Food::Poison
            }
        }, vec2(x, y)));
    }

    let frame_duration = Duration::from_secs_f32(1.0 / TARGET_FPS);

    loop {
        let frame_start = Instant::now();

        clear_background(BACKGROUND_COLOR);

        let mut o = 0;
        for i in 0..boids.len() {
            if boids[i-o].is_dead() {
                let boid = boids.swap_remove(i-o);
                o += 1;
                // println!("death...");

                food.push((Food::Apple, boid.vehicle.pos));
            } else {
                boids[i-o].boundaries((0.0..WINDOW_WIDTH as f32, 0.0..WINDOW_HEIGHT as f32));
                boids[i-o].eat(&mut food, DEBUG);
                boids[i-o].apply_friction();
                boids[i-o].update(1.0);
                boids[i-o].draw(Vec2::ZERO, DEBUG);

                if rng.random_bool(0.002) {
                    let new = boids[i-o].clone();
                    boids.push(new);
                    // println!("birth!");
                }
            }
        }

        if rng.random_bool(0.1) {
            let x = rng.random_range(0.0..WINDOW_WIDTH as f32);
            let y = rng.random_range(0.0..WINDOW_HEIGHT as f32);
            food.push((Food::Apple, vec2(x, y)));
        }
        if rng.random_bool(0.01) {
            let x = rng.random_range(0.0..WINDOW_WIDTH as f32);
            let y = rng.random_range(0.0..WINDOW_HEIGHT as f32);
            food.push((Food::Poison, vec2(x, y)));
        }

        for f in &food {
            f.0.draw(f.1);
        }

        draw_text(&format!("{}", get_fps()), 5.0, 25.0, 35.0, TEXT_COLOR);

        next_frame().await;

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            sleep(frame_duration - elapsed);
        }
    }
}

