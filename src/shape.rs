use crate::boid::FOOD_RADIUS;

use std::ops::Range;

use macroquad::{math::{Vec2, vec2}, color::Color, shapes::draw_circle};


pub enum Shape {
    Circle { radius: f32, color: Color }
}

impl Shape {
    pub fn draw(&self, pos: Vec2, dir: f32) {
        match self {
            &Self::Circle { radius, color } => draw_circle(pos.x, pos.y, radius, color)
        }
    }

    pub fn set_color(&mut self, color: Color) {
        match self {
            Self::Circle { color: color_, .. } => *color_ = color
        }
    }

    pub fn boundaries(&self, pos: Vec2, boundaries: (Range<f32>, Range<f32>)) -> Vec2 {
        let (w, h) = boundaries;
        let mut desired = Vec2::ZERO;

        match self {
            Self::Circle { radius, .. } => {
                let radius = 25.0;
                if pos.x - radius < w.start {
                    desired += vec2(1.0, 0.0);
                } else if pos.x + radius > w.end {
                    desired -= vec2(1.0, 0.0);
                }

                if pos.y - radius < h.start {
                    desired += vec2(0.0, 1.0);
                } else if pos.y + radius > h.end {
                    desired -= vec2(0.0, 1.0);
                }
            }
        }

        return desired;
    }

    pub fn collides_food(&self, pos: Vec2, food_pos: Vec2) -> bool {
        return match self {
            Self::Circle { radius, .. } => pos.distance_squared(food_pos) <= FOOD_RADIUS.powi(2)
        };
    }
}

pub fn color_lerp(color_a: Color, color_b: Color, t: f32) -> Color {
    return Color::from_vec(color_a.to_vec().move_towards(color_b.to_vec(), t));
}

