use macroquad::prelude::*;

const GRAVITY: f32 = -9.8;
const RADIUS: f32 = 30.0;

struct Bird {
    height: f32,
    x_position: f32,
    velocity: f32,
}

impl Bird {
    pub fn new() -> Bird {
        Bird {
            height: screen_height() / 2.0,
            x_position: screen_width() * 0.25,
            velocity: 0.0,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.x_position, self.height, RADIUS, PURPLE);
    }

    pub fn update(&mut self) {
        self.height += self.velocity;
        self.velocity -= get_frame_time() * GRAVITY;
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut bird = Bird::new();

    loop {
        clear_background(WHITE);
        draw_text("MOLLY BIRD!", 20.0, 20.0, 30.0, DARKGRAY);

        bird.draw();
        bird.update();

        next_frame().await
    }
}
