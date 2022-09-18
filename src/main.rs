
use macroquad::prelude::*;
mod biot_collection;
use biot_collection::{BiotCollection, Biot};
use rusqlite::{Connection, Result};

fn window_conf() -> Conf {
    Conf {
        window_title: "Yeast Simulation".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

fn initalize_database() {
    //Interface with sqlite
    let conn = Connection::open("simulation.db").unwrap();
    conn.execute(
        "create table if not exists simulation (
            id integer primary key,
            iteration integer,
            cells integer
        )",
        [],
    )
    .unwrap();
}

fn input_database(iteration: i32, cells: i32) {
    let conn = Connection::open("simulation.db").unwrap();
    conn.execute(
        "INSERT INTO simulation (iteration, cells) values (?1, ?2)",
        [iteration, cells],
    )
    .unwrap();
}
// fn query_db() -> Vec<i32> {
//     let mut query_cells = Vec<>::new();
//     let mut query_cells_out = Vec<>::new();
//     let conn = Connection::open("simulation.db").unwrap();
//     let mut stmt = conn.prepare("SELECT cells FROM simulation").unwrap();
//     let query_iter = stmt.query_map([], |row| {
//         Ok(query_cells)
//     }).unwrap();
//     for cell in query_iter {
//         query_cells_out.push(cell);
//     }
//     query_cells_out
// }


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


        if iterations % 1000 == 0 {
            initalize_database();
            input_database(iterations, biots.len() as i32);
        }
        next_frame().await
    }
}
