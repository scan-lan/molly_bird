use macroquad::prelude::*;

const GRAVITY: f32 = 9.8;
const RADIUS: f32 = 30.0;
const JUMP_VELOCITY: f32 = -4.5;

struct Bird {
    height: f32,
    velocity: f32,
}

fn get_x_position() -> f32 {
    screen_width() * 0.25
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

        if self.height > screen_height() {
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

#[macroquad::main("Molly Bird")]
async fn main() {
    let mut bird = Bird::new();

    loop {
        clear_background(WHITE);
        draw_text("MOLLY BIRD!", 20.0, 20.0, 30.0, DARKGRAY);

        if is_key_down(KeyCode::Space) {
            bird.jump();
        }

        bird.update();
        bird.draw();

        next_frame().await
    }
}
