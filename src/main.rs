use macroquad::prelude::*;
use graplot::Plot;

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

    rand::srand(miniquad::date::now().to_bits());
    let mut biots = BiotCollection::new(600);
    let mut iterations = 1;
    let plot = Plot::new([-4., -2., 1., 4.]);
    plot.show();

    loop {
        biots.step();
        clear_background(Color::new(0.,0.,0.1,1.0));
        biots.draw();
        draw_text(&format!("FPS: {}, Cells: {}, Iterations: {}", get_fps(), biots.len(), iterations),
            screen_width()-600., screen_height()-5.,
            28.,
            LIGHTGRAY);
        iterations += 1;

        next_frame().await
    }
}
