

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
            // More initial bacteria if needed
        ],
    };

    let mut iteration = 0;

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
            } else if (b.age % 10 == 0) && (b.energy > 20.0) && (collection_len < 100000) {
                let child = Bacteria::new(0, b.position, 50.0, b.genome.clone()); // New bacteria start with less energy
                new_bacteria.push(child);
            }
        }

        collection.bacteria.append(&mut new_bacteria);

        for index in to_remove.iter().rev() {
            collection.bacteria.remove(*index);
        }

        println!("Bacteria: {}", collection.bacteria.len());

    }
}
