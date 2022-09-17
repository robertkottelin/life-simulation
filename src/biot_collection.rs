use macroquad::prelude::*;
/// Modulus operator to get toroidal world topology
fn modulus<T>(a:T, b:T) -> T
where T: std::ops::Rem<Output=T>+
      std::ops::Add<Output = T>+
      Copy
{
    ((a % b) + b) % b
}
/// A single biot
#[derive(Clone, Debug)]
pub struct Biot {
    // Status
    life: f32,
    pub pos: Vec2,
    speed: Vec2,
}
impl Biot {
    /// Create a random biot
    pub fn random_biot() -> Self {
        let mut s = Self {
            life: 0.,
            pos: vec2(
                rand::gen_range(0., 1.)*screen_width(),
                rand::gen_range(0., 1.)*screen_height()
                ),
            speed: vec2(0., 0.),
        };
        // s.set_from_genome();
        s.life = s.base_life();
        s
    }

    pub fn step(&mut self) {
        if self.life >= 1f32 {
            BiotCollection::new(1);
            self.life -= 200f32;
        }
        self.pos += self.speed;
        self.pos.x = modulus(self.pos.x, screen_width());
        self.pos.y = modulus(self.pos.y, screen_height());
        self.speed *= 0.1;
        self.random_move(0.2);
    }

    fn random_move(&mut self, speed: f32) {
        self.accelerate(vec2(rand::gen_range(0., 1.)-0.5, rand::gen_range(0., 1.)-0.5).normalize(), speed);
    }
    /// Apply acceleration in a certain direction
    fn accelerate(&mut self, dir:Vec2, speed: f32) {
        self.speed += dir *speed;
    }

    fn base_life(&self) -> f32 {
        100f32
    }
}

/// A collection of biots. Responsible for handling interactions between biots
pub struct BiotCollection {
    biots: Vec<Biot>
}
impl BiotCollection {
    /// Create n random biots
    pub fn new(n: usize) -> Self {
        let mut s = Self { biots: Vec::new() };
        for _ in 0..n {
            s.biots.push(Biot::random_biot());
        }
        s
    }
    /// Compute one step of the simulation.
    pub fn step(&mut self) {
        let mut new : Vec<Biot> = Vec::new();

        for n in 0..(self.biots.len()) {
            self.biots[n].step();
        }
        self.biots.append(&mut new);
    }
    /// Display the biot collection
    pub fn draw(&self) {
        for biot in self.biots.iter() {
            draw_circle(biot.pos.x,biot.pos.y, 7., GREEN);
        }
    }
    /// The number of biots currently in our collection
    pub fn len(&self) -> usize {
        self.biots.len()
    }
}

