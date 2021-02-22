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
        for i1 in 0..self.particles.len() {
            self.particles[i1].accelerate(self.gravity);
            self.particles[i1].move_();
            self.particles[i1].bounce(self.width, self.height);
            for i2 in (i1 + 1)..self.particles.len() {
                let (a, b) = self.particles.split_at_mut(i2);
                Self::collide(&mut a[i1], &mut b[0]);
            }
        }
    }

    pub fn collide(p1: &mut Particle, p2: &mut Particle) {
        let dx = p1.position[0] - p2.position[0];
        let dy = p1.position[1] - p2.position[1];
        let distance = dx.hypot(dy);

        if distance < p1.size + p2.size {
            let normal = vecmath::vec2_normalized_sub(p1.position, p2.position);
            let scale1 = vecmath::vec2_dot(vecmath::vec2_sub(p1.velocity, p2.velocity), normal);
            let scale2 = vecmath::vec2_dot(vecmath::vec2_sub(p2.velocity, p1.velocity), normal);

            p1.velocity = vecmath::vec2_scale(normal, scale1 * p1.material.restitution);
            p2.velocity = vecmath::vec2_scale(normal, scale2 * p2.material.restitution);

            println!("collided!");
        }
    }
}
