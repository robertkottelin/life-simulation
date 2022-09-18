use macroquad::prelude::*;
mod biot_collection;
use biot_collection::{Biot, BiotCollection, Nutrition};
use rayon::prelude::*;
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
#[derive(Copy, Clone)]
struct CellIteration {
    cell_count: i32,
    iteration: i32,
}
enum MyError {
    DBError(rusqlite::Error),
    RowError(Vec<String>),
}
impl From<rusqlite::Error> for MyError {
    fn from(err: rusqlite::Error) -> Self {
        Self::DBError(err)
    }
}
fn query_db() -> Result<Vec<CellIteration>, MyError> {
    // let mut query_cells = Vec<>::new();
    // let mut query_cells_out = Vec<>::new();
    let conn = Connection::open("simulation.db").unwrap();
    let mut stmt = conn
        .prepare("SELECT iteration, cells FROM simulation")
        .unwrap();
    let query: Vec<Result<CellIteration, rusqlite::Error>> = stmt
        .query_map([], |row| {
            Ok(CellIteration {
                iteration: row.get(1)?,
                cell_count: row.get(0)?,
            })
        })?
        .collect();
    let errors = query
        .iter()
        .filter_map(|r| match r {
            Err(err) => Some(format!("Errors: {}", err)),
            Ok(_) => None,
        })
        .collect::<Vec<_>>();
    if !errors.is_empty() {
        return Err(MyError::RowError(errors));
    }
    let celliterations = query
        .par_iter()
        .filter_map(|r| match r {
            Ok(ci) => Some(*ci),
            _ => None,
        })
        .collect();
    Ok(celliterations)
}
#[macroquad::main(window_conf())]
async fn main() {
    let mut biots = BiotCollection::new(10);
    let nutrition = Nutrition::new();
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
        nutrition.draw();
        biots.draw();


        if iterations % 1000 == 0 {
            initalize_database();
            input_database(iterations, biots.len() as i32);
        }
        next_frame().await
    }
}
