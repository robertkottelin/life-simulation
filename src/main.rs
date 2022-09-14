use macroquad::prelude::*;
// use graplot::Plot;
mod biot;
mod biot_collection;
use biot_collection::BiotCollection;

struct Textbox {
    rect: Rect,
}
impl Textbox {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.0f32,
                screen_height() * 0.0f32,
                2000f32,
                22f32,
            ),
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Life Simulation".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {

    rand::srand(miniquad::date::now().to_bits());
    let mut biots = BiotCollection::new(600);
    let mut iterations = 1;
    let textbox = Textbox::new();
    // let plot = Plot::new([-4., -2., 1., 4.]);
    // plot.show();

    loop {
        biots.step();
        iterations += 1;
        clear_background(Color::new(0.30, 0.25, 0.16, 1.00));
        biots.draw();
        textbox.draw();
        draw_text(&format!("FPS: {}, Cells: {}, Iterations: {}", get_fps(), biots.len(), iterations),
            screen_width()*0.01f32, screen_height()*0.028f32,
            22.,
            WHITE);
        next_frame().await
    }
}
