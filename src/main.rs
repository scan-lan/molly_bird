use std::collections::VecDeque;

use macroquad::{prelude::*, rand};

const GRAVITY: f32 = 20.;
const RADIUS: f32 = 20.0;
const CIRCUMFERENCE: f32 = RADIUS * 2.;
const JUMP_VELOCITY: f32 = -5.;

fn get_x_position() -> f32 {
    screen_width() / 3.0
}

struct Bird {
    height: f32,
    velocity: f32,
}

impl Bird {
    pub fn new() -> Bird {
        Bird {
            height: screen_height() / 2.0,
            velocity: 0.0,
        }
    }

    pub fn tick(&mut self) {
        self.update();
        self.draw();
    }

    fn draw(&self) {
        draw_circle(get_x_position(), self.height, RADIUS, PURPLE);
    }

    fn update(&mut self) {
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
    x: f32,
    gap_height: f32,
}

impl Obstacle {
    pub fn new() -> Self {
        Obstacle {
            x_offset: 10.,
            x: screen_width() + 10.,
            gap_height: rand::gen_range(GAP_SIZE, screen_height() - GAP_SIZE),
        }
    }

    pub fn draw(&self) {
        let t_rect_h = self.gap_height - (GAP_SIZE / 2.0);
        // Top rectangle
        draw_rectangle(self.x, 0.0, OBSTACLE_WIDTH, t_rect_h, DARKGRAY);

        let b_rect_start = self.gap_height + GAP_SIZE / 2.0;
        let b_rect_h = screen_height() - (self.gap_height + GAP_SIZE / 2.0);

        // Bottom rectangle
        draw_rectangle(self.x, b_rect_start, OBSTACLE_WIDTH, b_rect_h, DARKGRAY);
    }

    pub fn update(&mut self) {
        self.x_offset -= get_frame_time() * OBSTACLE_SPEED;
        self.x = screen_width() + self.x_offset;
    }
}

const OBSTACLE_GAP: f32 = 300.;
const GAP_SIZE: f32 = CIRCUMFERENCE * 3.8;
const OBSTACLE_WIDTH: f32 = CIRCUMFERENCE * 2.;
const OBSTACLE_SPEED: f32 = 250.0;

struct Obstacles {
    list: VecDeque<Obstacle>,
}

impl Obstacles {
    pub fn new() -> Self {
        let mut obstacles = VecDeque::new();
        obstacles.push_back(Obstacle::new());

        Self { list: obstacles }
    }

    pub fn tick(&mut self) {
        self.update();
        self.draw();
    }

    fn draw(&self) {
        self.list.iter().for_each(|obs| obs.draw())
    }

    fn update(&mut self) {
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

#[macroquad::main("Molly Bird")]
async fn main() {
    let mut fps_hist = VecDeque::<i32>::new();
    let bg_color = color_u8!(139, 184, 232, 255);
    let mut bird = Bird::new();
    let mut obstacles = Obstacles::new();
    let mut paused = false;
    let mut can_pause = true;

    loop {
        if fps_hist.len() == 20 {
            fps_hist.pop_front();
        }
        fps_hist.push_back(get_fps());
        let fps: i32 = fps_hist.iter().sum::<i32>() / fps_hist.len() as i32;
        let fps_str = fps.to_string();
        clear_background(bg_color);
        if let Some(action) = handle_input() {
            match action {
                Action::Jump => bird.jump(),
                Action::Pause => {
                    if can_pause {
                        paused = !paused;
                        can_pause = false;
                    }
                }
                Action::PauseReleased => can_pause = true,
                Action::Reset => {
                    bird.reset();
                    // obstacles.reset()
                }
            };
        }

        if !paused {
            obstacles.tick();
            bird.tick();
        } else {
            obstacles.draw();
            bird.draw();
            draw_pause_menu();
        }

        draw_text("MOLLY BIRD!", 20.0, 25.0, 40.0, PINK);
        draw_text(
            &fps_str,
            screen_width() - 60.,
            screen_height() - 20.,
            40.,
            PINK,
        );

        next_frame().await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Jump,
    Reset,
    Pause,
    PauseReleased,
}

fn handle_input() -> Option<Action> {
    let keys = get_keys_down();

    if is_key_released(KeyCode::Escape) {
        return Some(Action::PauseReleased);
    }

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

const PAUSE_SIGN_WIDTH: f32 = 100.;
const PAUSE_SIGN_HEIGHT: f32 = PAUSE_SIGN_WIDTH * 3.;
const PAUSE_SIGN_GAP: f32 = PAUSE_SIGN_WIDTH * 0.7;

fn draw_pause_menu() {
    let middle_screen_x = screen_width() / 2.;
    let middle_screen_y = screen_height() / 2. - PAUSE_SIGN_HEIGHT / 2.;

    // Draw background
    draw_rectangle(
        0.,
        0.,
        screen_width(),
        screen_height(),
        color_u8!(255, 255, 255, 100),
    );

    // Draw pause symbol
    draw_rectangle(
        middle_screen_x - PAUSE_SIGN_WIDTH - PAUSE_SIGN_GAP / 2.,
        middle_screen_y,
        PAUSE_SIGN_WIDTH,
        PAUSE_SIGN_HEIGHT,
        DARKGRAY,
    );
    draw_rectangle(
        middle_screen_x + PAUSE_SIGN_GAP / 2.,
        middle_screen_y,
        PAUSE_SIGN_WIDTH,
        PAUSE_SIGN_HEIGHT,
        DARKGRAY,
    );
}
