use std::collections::HashMap;
use plotters::prelude::*;

#[derive(Clone, Debug)]
struct Bacteria {
    age: u32,
    position: (i32, i32, i32),
    energy: f32,
    genome: Vec<u8>,
}

impl Bacteria {
    fn new(age: u32, position: (i32, i32, i32), energy: f32, genome: Vec<u8>) -> Bacteria {
        Bacteria {
            age,
            position,
            energy,
            genome,
        }
    }
}

#[derive(Debug)]
struct Collection {
    bacteria: Vec<Bacteria>,
}

fn main() {
    let mut collection = Collection {
        bacteria: vec![
            Bacteria::new(0, (0, 0, 0), 100.0, vec![0, 1, 0, 1]),
            Bacteria::new(0, (1, 1, 1), 100.0, vec![1, 0, 1, 0]),
        ],
    };

    let mut iteration = 0;
    let mut bacteria_count_map: HashMap<u32, usize> = HashMap::new();

    loop {
        let mut to_remove = vec![];
        let mut new_bacteria = vec![];
        iteration += 1;
        print!("Iteration: {} ", iteration);

        let collection_len = collection.bacteria.len();

        for (index, b) in collection.bacteria.iter_mut().enumerate() {
            b.age += 1;
            b.energy -= 0.2;

            if b.energy <= 0.0 {
                to_remove.push(index);
            } else if (b.age % 10 == 0) && (b.energy > 1.0) && (collection_len < 100000) {
                let child = Bacteria::new(0, b.position, 100.0, b.genome.clone()); // New bacteria start with less energy
                new_bacteria.push(child);
            }
        }

        collection.bacteria.append(&mut new_bacteria);

        for index in to_remove.iter().rev() {
            collection.bacteria.remove(*index);
        }

        println!("Bacteria: {}", collection.bacteria.len());
        
        // Update the hashmap with the current iteration and bacteria count
        bacteria_count_map.insert(iteration, collection.bacteria.len());

        if iteration == 1000 {
            break;
        }
    }

    // plot the history of bacteria count from the hashmap

    let mut data: Vec<(u32, usize)> = bacteria_count_map.into_iter().collect();
    data.sort_by_key(|&(iteration, _)| iteration);

    // Define the dimensions of the plot
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    // Create a drawing area
    let root = BitMapBackend::new("bacteria_plot.png", (WIDTH, HEIGHT))
        .into_drawing_area();

    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("y: Bacteria Count x: Iterations", ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..1000, 0..160000)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // Convert sorted data to a vector of points for plotting
    let data_points: Vec<(i32, i32)> = data
        .iter()
        .map(|&(k, v)| (k as i32, v as i32))
        .collect();

    // Draw the line
    chart
        .draw_series(LineSeries::new(
            data_points.into_iter(),
            &RED,
        ))
        .unwrap();

    // Save the plot
    root.present().unwrap();
    println!("Plot saved to bacteria_plot.png");
}
