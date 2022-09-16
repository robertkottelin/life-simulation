use macroquad::prelude::*;
// use graplot::Plot;
mod biot;
mod biot_collection;
use biot_collection::BiotCollection;

fn window_conf() -> Conf {
    Conf {
        window_title: "Life Simulation".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf())]
async fn main() {
    let mut biots = BiotCollection::new(60);
    let mut iterations = 1;

    loop {
        biots.step();
        clear_background(Color::new(0.30, 0.25, 0.16, 1.00));
        biots.draw();
        draw_text(&format!("FPS: {}, Cells: {}, Iterations: {}", get_fps(), biots.len(), iterations),
            screen_width()*0.01f32, screen_height()*0.02f32,
            22.,
            WHITE);
        iterations += 1;
        next_frame().await
    }
}
