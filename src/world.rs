use vecmath::Vector2;

use crate::particle::Particle;

#[derive(Debug)]
pub struct World {
    pub width: usize,
    pub height: usize,
    pub gravity: Vector2<f64>,

    pub particles: Vec<Particle>,
}

impl World {
    pub fn new(width: usize, height: usize, gravity: Vector2<f64>) -> Self {
        Self {
            width,
            height,
            gravity,

            particles: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, p: Particle) {
        self.particles.push(p);
    }

    pub fn update(&mut self) {
        for p in self.particles.iter_mut() {
            p.accelerate(self.gravity);
            p.move_();
            p.bounce(self.width, self.height);
        }
    }
}
