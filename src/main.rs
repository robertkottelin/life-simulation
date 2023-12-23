
#[derive(Clone, Debug)]
struct Bacteria {
    name: String,
    age: u32,
    position: (i32, i32, i32),
    energy: f32,
    genome: Vec<u8>,
}

impl Bacteria {
    fn new(name: String, age: u32, position: (i32, i32, i32), energy: f32, genome: Vec<u8>) -> Bacteria {
        Bacteria {
            name,
            age,
            position,
            energy,
            genome,
        }
    }
}

fn main() {
    let mut bacteria = Bacteria::new(
        String::from("E. coli"),
        0,
        (0, 0, 0),
        100.0,
        vec![0, 1, 0, 1],
    );

    loop {
        bacteria.age += 1;
        bacteria.energy -= 0.1;

        if bacteria.energy <= 0.0 {
            break;
        }
    }
}