use std::collections::VecDeque;

use macroquad::{prelude::*, rand};

const GRAVITY: f32 = 9.8;
const RADIUS: f32 = 25.0;
const JUMP_VELOCITY: f32 = -4.5;
const GAP_SIZE: f32 = (RADIUS + 7.) * 2.;
const OBSTACLE_WIDTH: f32 = 30.0;

struct Bird {
    height: f32,
    velocity: f32,
}

fn get_x_position() -> f32 {
    screen_width() / 3.0
}

impl Bird {
    pub fn new() -> Bird {
        Bird {
            height: screen_height() / 2.0,
            velocity: 0.0,
        }
    }

    pub fn draw(&self) {
        draw_circle(get_x_position(), self.height, RADIUS, PURPLE);
    }

    pub fn update(&mut self) {
        self.height += self.velocity;
        self.velocity += get_frame_time() * GRAVITY;

        if self.height > screen_height() && self.velocity < 0. {
            self.reset();
        }
    }

    pub fn jump(&mut self) {
        self.velocity = JUMP_VELOCITY;
    }

    pub fn reset(&mut self) {
        self.height = screen_height() / 2.0;
        self.velocity = 0.0;
    }
}

struct Obstacle {
    x_position: f32,
    gap_height: f32,
}

impl Obstacle {
    pub fn new() -> Self {
        Obstacle {
            x_position: screen_width() - 100.0,
            gap_height: rand::gen_range(40.0, screen_height() - 40.0),
        }
    }

    pub fn draw(&self) {
        let t_rect_h = self.gap_height - (GAP_SIZE / 2.0);
        // Top rectangle
        draw_rectangle(self.x_position, 0.0, OBSTACLE_WIDTH, t_rect_h, DARKGRAY);

        let b_rect_start = self.gap_height + GAP_SIZE / 2.0;
        let b_rect_h = screen_height() - (self.gap_height + GAP_SIZE / 2.0);

        // Bottom rectangle
        draw_rectangle(
            self.x_position,
            b_rect_start,
            OBSTACLE_WIDTH,
            b_rect_h,
            DARKGRAY,
        );
    }
}

#[macroquad::main("Molly Bird")]
async fn main() {
    let mut bird = Bird::new();
    let mut obstacles = VecDeque::new();
    obstacles.push_back(Obstacle::new());

    loop {
        clear_background(WHITE);
        draw_text("MOLLY BIRD!", 20.0, 20.0, 30.0, DARKGRAY);

        if is_key_down(KeyCode::Space) {
            bird.jump();
        }

        bird.update();
        bird.draw();

        obstacles.iter().for_each(|obstacle| {
            obstacle.draw();
            // obstacle.update();
        });

        next_frame().await
    }
}
