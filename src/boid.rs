use crate::shape::{Shape, color_lerp};

use std::ops::Range;

use ::rand::{Rng, RngExt};
use macroquad::{math::{Vec2, vec2}, shapes::{draw_circle, draw_circle_lines, draw_line}, text::draw_text, color::Color};


pub struct Vehicle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub shape: Shape,
    pub weight: f32
}

impl Vehicle {
    pub fn update(&mut self, dt: f32) {
        self.vel += self.acc / self.weight * dt;
        self.pos += self.vel * dt;
        self.acc = Vec2::ZERO;
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acc += force;
    }

    pub fn draw(&self, cam: Vec2) {
        self.shape.draw(self.pos - cam, self.vel.normalize_or_zero());
    }
}

pub const BOID_MAX_SPEED: f32 = 5.0;
pub const BOID_MAX_FORCE: f32 = 0.5;
pub const BOID_FRICTION: f32 = 0.005;
pub const BOID_WEIGHT: f32 = 1.0;
pub const BOID_HEALTH: Range<f32> = 0.0..1.0;
pub const BOID_HEALTH_DEPLETION_RATE: f32 = 0.003;
pub const BOID_RADIUS: f32 = 16.0;
pub const BOID_COLOR_A: Color = Color::from_rgba(127, 255, 0, 255);
pub const BOID_COLOR_B: Color = Color::from_rgba(255, 0, 0, 255);
// pub const BOID_SHAPE: Shape = Shape::Circle { radius: BOID_RADIUS, color: BOID_COLOR_A };
pub const BOID_SHAPE: Shape = Shape::Triange { size: BOID_RADIUS, color: BOID_COLOR_A };

pub struct Boid {
    pub vehicle: Vehicle,
    pub dna: Dna,
    pub health: f32,
    pub max_speed: f32,
    pub max_force: f32
}

impl Boid {
    pub fn new(pos: Vec2) -> Self {
        return Self {
            vehicle: Vehicle {
                pos: pos,
                vel: {
                    let mut rng = ::rand::rng();
                    let vel = vec2(rng.random_range(0.0..1.0), rng.random_range(0.0..1.0));
                    vel.normalize()
                },
                acc: Vec2::ZERO,
                shape: BOID_SHAPE,
                weight: BOID_WEIGHT
            },
            dna: Dna::initial(),
            health: BOID_HEALTH.end,
            max_speed: BOID_MAX_SPEED,
            max_force: BOID_MAX_FORCE
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.health -= BOID_HEALTH_DEPLETION_RATE * dt;
        self.vehicle.shape.set_color(color_lerp(BOID_COLOR_A, BOID_COLOR_B, 1.0 - self.health.clamp(0.0, 1.0)));

        self.vehicle.vel += self.vehicle.acc / self.vehicle.weight * dt;
        self.vehicle.vel = self.vehicle.vel.normalize_or_zero() * self.vehicle.vel.length().min(self.max_speed);
        self.vehicle.pos += self.vehicle.vel * dt;
        self.vehicle.acc = Vec2::ZERO;
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.vehicle.apply_force(force);
    }

    pub fn seek(&self, target: Vec2) -> Vec2 {
        let desired = (target - self.vehicle.pos).normalize_or_zero() * self.max_speed;
        let mut steer = desired - self.vehicle.vel;
        steer = steer.normalize_or_zero() * steer.length().min(self.max_force);

        return steer;
    }

    pub fn eat(&mut self, food: &mut Vec<(Food, Vec2)>, debug: bool) {
        let mut closest_apple: Option<(usize, f32)> = None;
        let mut closest_poison: Option<(usize, f32)> = None;
        let mut removed: Vec<usize> = Vec::new();

        for i in 0..food.len() {
            if self.vehicle.shape.collides_food(self.vehicle.pos, food[i].1) {
                self.health += food[i].0.nutrition();

                removed.push(i);
            } else {
                let dist2 = self.vehicle.pos.distance_squared(food[i].1);
                let mut best = false;

                match food[i].0 {
                    Food::Apple => {
                        if dist2 < self.dna.apple_perception.powi(2) {
                            if let Some((_, dist2_)) = closest_apple {
                                best = dist2 < dist2_;
                            } else {
                                best = true;
                            }
                        }

                        if best {
                            closest_apple = Some((i, dist2));
                        }
                    },
                    Food::Poison => {
                        if dist2 < self.dna.apple_perception.powi(2) {
                            if let Some((_, dist2_)) = closest_apple {
                                best = dist2 < dist2_;
                            } else {
                                best = true;
                            }
                        }


                        if best {
                            closest_poison = Some((i, dist2));
                        }
                    }
                }
            }
        }

        let apple_desired = if let Some((index, _)) = closest_apple {
            self.seek(food[index].1)
        } else { Vec2::ZERO };
        let apple_steer = apple_desired * self.dna.apple_weight;
        self.apply_force(apple_steer);

        let poison_desired = if let Some((index, _)) = closest_poison {
            self.seek(food[index].1)
        } else { Vec2::ZERO };
        let poison_steer = poison_desired * self.dna.poison_weight;
        self.apply_force(poison_steer);

        for (i, index) in removed.into_iter().enumerate() {
            food.swap_remove(index-i);
        }

        if debug && false {
            let (x, y) = self.vehicle.pos.into();
            let (ax, ay) = (apple_steer * 15.0).into();
            let (px, py) = (poison_steer * 15.0).into();
            draw_line(x, y, x+ax, y+ay, 1.0, Color::from_rgba(255, 255, 255, 255));
            draw_line(x, y, x+px, y+py, 1.0, Color::from_rgba(0, 0, 0, 255));
        }
    }

    pub fn boundaries(&mut self, boundaries: (Range<f32>, Range<f32>)) {
        let mut desired = self.vehicle.vel + self.vehicle.shape.boundaries(self.vehicle.pos, boundaries) * self.max_speed;
        desired = desired.normalize_or_zero() * self.max_speed;
        let mut steer = desired - self.vehicle.vel;
        steer = steer.normalize_or_zero() * steer.length().min(self.max_force);

        self.apply_force(steer);
    }

    pub fn apply_friction(&mut self) {
        let friction = -BOID_FRICTION * self.vehicle.vel;

        self.apply_force(friction);
    }

    pub fn clone(&self) -> Self {
        let mut new = Self::new(self.vehicle.pos);
        new.dna = self.dna.mutate();

        return new;
    }

    pub fn is_dead(&self) -> bool {
        return self.health <= BOID_HEALTH.start;
    }

    pub fn draw(&self, cam: Vec2, debug: bool) {
        self.vehicle.draw(cam);

        if debug {
            let (x, y) = self.vehicle.pos.into();
            let (vx, vy) = (self.vehicle.vel * 10.0).into();

            // draw_line(x, y, x+vx, y+vy, 2.0, Color::from_rgba(245, 0, 179, 255));
            draw_circle_lines(x, y, self.dna.apple_perception, 1.0, APPLE_COLOR);
            draw_circle_lines(x, y, self.dna.poison_perception, 1.0, POISON_COLOR);
            draw_text(&format!("{:.4}", self.health), x - 13.0, y + 20.0, 10.0, Color::from_rgba(245, 0, 179, 255));
        }
    }
}

pub const APPLE_WEIGHT_INITIAL: Range<f32> = 0.2..2.0;
pub const APPLE_WEIGHT_MUTATION: Range<f32> = -0.1..0.1;
pub const POISON_WEIGHT_INITIAL: Range<f32> = -1.5..0.1;
pub const POISON_WEIGHT_MUTATION: Range<f32> = -0.1..0.1;
pub const APPLE_PERCEPTION_INITIAL: Range<f32> = 100.0..800.0;
pub const APPLE_PERCEPTION_MUTATION: Range<f32> = -10.0..10.0;
pub const POISON_PERCEPTION_INITIAL: Range<f32> = 100.0..800.0;
pub const POISON_PERCEPTION_MUTATION: Range<f32> = -10.0..10.0;

pub const DNA_MUTATION_RATE: f64 = 0.2;

pub struct Dna {
    pub apple_weight: f32,
    pub poison_weight: f32,
    pub apple_perception: f32,
    pub poison_perception: f32
}

impl Dna {
    pub fn initial() -> Self {
        let mut rng = ::rand::rng();

        return Self {
            apple_weight: rng.random_range(APPLE_WEIGHT_INITIAL),
            poison_weight: rng.random_range(POISON_WEIGHT_INITIAL),
            apple_perception: rng.random_range(APPLE_PERCEPTION_INITIAL),
            poison_perception: rng.random_range(POISON_PERCEPTION_INITIAL)
        };
    }

    pub fn mutate(&self) -> Self {
        let mut rng = ::rand::rng();

        return Self {
            apple_weight: self.apple_weight + rng.random_range(APPLE_WEIGHT_MUTATION) * rng.random_bool(DNA_MUTATION_RATE) as u8 as f32,
            poison_weight: self.poison_weight + rng.random_range(POISON_WEIGHT_MUTATION),
            apple_perception: self.apple_perception + rng.random_range(APPLE_PERCEPTION_MUTATION),
            poison_perception: self.poison_perception + rng.random_range(POISON_PERCEPTION_MUTATION)
        };
    }
}

pub const APPLE_NUTRITION: f32 = 0.2;
pub const POISON_NUTRITION: f32 = -1.0;
pub const FOOD_RADIUS: f32 = 8.0;
pub const APPLE_COLOR: Color = Color::from_rgba(117, 167, 67, 255);
pub const POISON_COLOR: Color = Color::from_rgba(207, 87, 60, 255);

pub enum Food {
    Apple,
    Poison
}

impl Food {
    pub fn nutrition(&self) -> f32 {
        return match self {
            Self::Apple => APPLE_NUTRITION,
            Self::Poison => POISON_NUTRITION
        };
    }

    pub fn draw(&self, pos: Vec2) {
        let color = match self {
            Self::Apple => APPLE_COLOR,
            Self::Poison => POISON_COLOR
        };

        draw_circle(pos.x, pos.y, FOOD_RADIUS, color);
    }
}

