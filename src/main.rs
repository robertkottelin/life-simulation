use macroquad::prelude::*;
mod biot_collection;
use biot_collection::{BiotCollection};

fn window_conf() -> Conf {
    Conf {
        window_title: "Yeast Simulation".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf())]
async fn main() {
    let mut biots = BiotCollection::new(10);
    let mut iterations = 1;

    loop {
        biots.step();
        clear_background(Color::new(0.30, 0.25, 0.16, 1.00));
        draw_text(&format!("FPS: {}, Cells: {}, Iterations: {}", get_fps(), biots.len(), iterations),
            screen_width()*0.01f32, screen_height()*0.02f32,
            22.,
            WHITE);
        iterations += 1;
        biots.draw();
        next_frame().await
    }
}
