use macroquad::prelude::*;
use rayon::prelude::*;
mod modulus;

// Implement trait to render biots
trait RenderEntity {
    fn render(&self);
}

/// Nutrition
#[derive(Clone, Debug)]
pub struct Nutrition {
    nutrition_conc: f32,
    pub pos: Vec2,
}
impl RenderEntity for Nutrition {
    fn render(&self) {
        draw_circle(self.pos.x, self.pos.y, 100., GREEN);
    }
}
impl Nutrition {
    /// Create n random biots
    pub fn new() -> Self {
        let nutrition = Self {
            nutrition_conc: 100f32,
            pos: vec2(
                rand::gen_range(0., 1.) * screen_width(),
                rand::gen_range(0., 1.) * screen_height(),
            ),
        };
        nutrition
    }
    /// Display the nutrition
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 200., LIGHTGRAY);
    }
}

/// A single biot
#[derive(Clone, Debug)]
pub struct Biot {
    life: f32,
    pub pos: Vec2,
    speed: Vec2,
}

impl RenderEntity for Biot {
    fn render(&self) {
        draw_circle(self.pos.x, self.pos.y, 4., GREEN);
    }
}

impl Biot {
    /// Create a random biot
    pub fn random_biot() -> Self {
        let mut s = Self {
            life: 0.,
            pos: vec2(
                rand::gen_range(0., 1.) * screen_width(),
                rand::gen_range(0., 1.) * screen_height(),
            ),
            speed: vec2(0., 0.),
        };
        // s.set_from_genome();
        s.life = s.base_life();
        s
    }
    pub fn step(&mut self) {
        self.pos += self.speed;
        self.pos.x = modulus::modulus(self.pos.x, screen_width());
        self.pos.y = modulus::modulus(self.pos.y, screen_height());
        self.speed *= 0.2;
        self.random_move(0.2);
        self.life += 1.0;
    }
    fn random_move(&mut self, speed: f32) {
        self.accelerate(
            vec2(rand::gen_range(0., 1.) - 0.5, rand::gen_range(0., 1.) - 0.5).normalize(),
            speed,
        );
    }
    /// Apply acceleration in a certain direction
    fn accelerate(&mut self, dir: Vec2, speed: f32) {
        self.speed += dir * speed;
    }
    fn base_life(&self) -> f32 {
        1f32
    }
}

/// The collection of biots.
pub struct BiotCollection {
    biots: Vec<Biot>,
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
        let mut offspring = Vec::new();
        let mut new: Vec<Biot> = Vec::new();
        for n in 0..(self.biots.len()) {
            self.biots[n].step();
            if self.biots.len() <= 20000 {
                if (self.biots[n].life % 1000.0) == 0.0 {
                    let mut off = self.biots[n].clone();
                    off.life = 1f32;
                    off.random_move(2.0);
                    offspring.push(off);
                }
            }
            self.biots[n].life += 1f32;
        }
        self.biots.append(&mut new);
        self.biots.append(&mut offspring);
    }
    /// Display the biot collection
    pub fn draw(&self) {
        self.biots.par_iter().for_each(RenderEntity::render);
        // for biot in self.biots.iter() {
        //     draw_circle(biot.pos.x, biot.pos.y, 4., GREEN);
        // }
    }
    /// The number of biots currently in our collection
    pub fn len(&self) -> i32 {
        self.biots.len() as i32
    }
}
