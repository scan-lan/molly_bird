use std::collections::VecDeque;

use macroquad::{prelude::*, rand};

const GRAVITY: f32 = 20.;
const RADIUS: f32 = 20.0;
const CIRCUMFERENCE: f32 = RADIUS * 2.;
const JUMP_VELOCITY: f32 = -5.;
const GAP_SIZE: f32 = CIRCUMFERENCE * 3.8;
const OBSTACLE_WIDTH: f32 = CIRCUMFERENCE * 2.;
const OBSTACLE_SPEED: f32 = 250.0;

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
    x_offset: f32,
    gap_height: f32,
}

impl Obstacle {
    pub fn new() -> Self {
        Obstacle {
            x_offset: 10.,
            gap_height: rand::gen_range(GAP_SIZE, screen_height() - GAP_SIZE),
        }
    }

    pub fn draw(&self) {
        let t_rect_h = self.gap_height - (GAP_SIZE / 2.0);
        let x_pos = screen_width() + self.x_offset;
        // Top rectangle
        draw_rectangle(x_pos, 0.0, OBSTACLE_WIDTH, t_rect_h, DARKGRAY);

        let b_rect_start = self.gap_height + GAP_SIZE / 2.0;
        let b_rect_h = screen_height() - (self.gap_height + GAP_SIZE / 2.0);

        // Bottom rectangle
        draw_rectangle(x_pos, b_rect_start, OBSTACLE_WIDTH, b_rect_h, DARKGRAY);
    }

    pub fn update(&mut self) {
        self.x_offset -= get_frame_time() * OBSTACLE_SPEED;
    }
}

struct Obstacles {
    list: VecDeque<Obstacle>,
}

const OBSTACLE_GAP: f32 = 300.;

impl Obstacles {
    pub fn new() -> Self {
        let mut obstacles = VecDeque::new();
        obstacles.push_back(Obstacle::new());

        Self { list: obstacles }
    }

    pub fn draw(&self) {
        self.list.iter().for_each(|obs| obs.draw())
    }

    pub fn update(&mut self) {
        if let Some(front) = self.list.front() {
            if front.x_offset <= -(screen_width() + OBSTACLE_WIDTH) {
                self.list.pop_front();
            }
        }

        if let Some(back) = self.list.back() {
            if back.x_offset < -OBSTACLE_GAP {
                self.list.push_back(Obstacle::new())
            }
        } else {
            self.list.push_back(Obstacle::new())
        }

        self.list.iter_mut().for_each(|obs| obs.update())
    }
}

enum Action {
    Jump,
    Reset,
    Pause,
}

fn handle_input() -> Option<Action> {
    let keys = get_keys_down();

    if keys.contains(&KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
        Some(Action::Jump)
    } else if keys.contains(&KeyCode::Escape) {
        Some(Action::Pause)
    } else if keys.contains(&KeyCode::R) {
        Some(Action::Reset)
    } else {
        None
    }
}

#[macroquad::main("Molly Bird")]
async fn main() {
    let mut bird = Bird::new();
    let mut obstacles = Obstacles::new();
    let mut paused = true;

    loop {
        let fps = get_fps().to_string();
        clear_background(WHITE);
        draw_text("MOLLY BIRD!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(&fps, screen_width() - 100., 20., 30., DARKGRAY);

        if let Some(action) = handle_input() {
            match action {
                Action::Jump => bird.jump(),
                Action::Pause => paused = !paused,
                Action::Reset => {
                    bird.reset();
                    // obstacles.reset()
                }
            };
        }

        bird.update();
        obstacles.update();
        bird.draw();
        obstacles.draw();

        next_frame().await
    }
}
