use rayon::prelude::*;
use rusqlite::{Connection, Result};

pub fn initalize_database() {
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
pub fn input_database(iteration: i32, cells: i32) {
    let conn = Connection::open("simulation.db").unwrap();
    conn.execute(
        "INSERT INTO simulation (iteration, cells) values (?1, ?2)",
        [iteration, cells],
    )
    .unwrap();
}
#[derive(Copy, Clone)]
pub struct CellIteration {
    cell_count: i32,
    iteration: i32,
}
pub enum MyError {
    DBError(rusqlite::Error),
    RowError(Vec<String>),
}
impl From<rusqlite::Error> for MyError {
    fn from(err: rusqlite::Error) -> Self {
        Self::DBError(err)
    }
}
pub fn query_db() -> Result<Vec<CellIteration>, MyError> {
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