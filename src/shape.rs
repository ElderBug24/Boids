use crate::boid::FOOD_RADIUS;

use std::ops::Range;

use macroquad::{math::{Vec2, vec2}, color::Color, shapes::{draw_circle, draw_triangle}};


pub enum Shape {
    Circle { radius: f32, color: Color },
    Triange { size: f32, color: Color }
}

impl Shape {
    pub fn draw(&self, pos: Vec2, dir: Vec2) {
        match self {
            &Self::Circle { radius, color } => draw_circle(pos.x, pos.y, radius, color),
            &Self::Triange { size, color } => {
                let a = pos + dir.rotate(vec2(size, 0.0));
                let b = pos + dir.rotate(vec2(-size, -0.5 * size));
                let c = pos + dir.rotate(vec2(-size, 0.5 * size));

                draw_triangle(a, b, c, color);
                // draw_circle(pos.x, pos.y, size, Color::from_rgba(255, 255, 255, 255));
            }
        }
    }

    pub fn set_color(&mut self, color: Color) {
        match self {
            Self::Circle { color: color_, .. } => *color_ = color,
            Self::Triange { color: color_, .. } => *color_ = color
        }
    }

    pub fn boundaries(&self, pos: Vec2, boundaries: (Range<f32>, Range<f32>)) -> Vec2 {
        let (w, h) = boundaries;
        let mut desired = Vec2::ZERO;

        let dist = match self {
            Self::Circle { radius, .. } => radius,
            Self::Triange { size, .. } => size
        };

        if pos.x - dist < w.start {
            desired += vec2(1.0, 0.0);
        } else if pos.x + dist > w.end {
            desired -= vec2(1.0, 0.0);
        }

        if pos.y - dist < h.start {
            desired += vec2(0.0, 1.0);
        } else if pos.y + dist > h.end {
            desired -= vec2(0.0, 1.0);
        }

        return desired;
    }

    pub fn collides_food(&self, pos: Vec2, food_pos: Vec2) -> bool {
        let dist = match self {
            Self::Circle { radius, .. } => radius,
            Self::Triange { size, .. } => size
        };

        return pos.distance_squared(food_pos) <= (FOOD_RADIUS + dist).powi(2);
    }
}

pub fn color_lerp(color_a: Color, color_b: Color, t: f32) -> Color {
    return Color::from_vec(color_a.to_vec().move_towards(color_b.to_vec(), t));
}

