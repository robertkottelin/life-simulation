
use macroquad::prelude::*;
mod biot_collection;
use biot_collection::BiotCollection;
use rusqlite::{Connection};


fn window_conf() -> Conf {
    Conf {
        window_title: "Yeast Simulation".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}
#[allow(dead_code)]
fn initalize_database() {
    //Interface with sqlite
    let conn = Connection::open("simulation.db").unwrap();
    conn.execute(
        "create table if not exists simulation (
            id integer primary key,
            iteration integer,
            cells integer,
        )",
        [],
    )
    .unwrap();
}
#[allow(dead_code)]
fn input_database(iteration: i32, cells: i32) {
    let conn = Connection::open("gamestate.db").unwrap();
    conn.execute(
        "INSERT INTO simulation (iteration, cells) values (?1, ?2)",
        [iteration, cells],
    )
    .unwrap();
}
#[macroquad::main(window_conf())]
async fn main() {
    let mut biots = BiotCollection::new(10);
    let mut iterations = 1;

    loop {
        biots.step();
        clear_background(Color::new(0.30, 0.25, 0.16, 1.00));
        draw_text(
            &format!(
                "FPS: {}, Cells: {}, Iterations: {}",
                get_fps(),
                biots.len(),
                iterations
            ),
            screen_width() * 0.01f32,
            screen_height() * 0.02f32,
            22.,
            WHITE,
        );
        iterations += 1;
        biots.draw();
        next_frame().await
    }
}
