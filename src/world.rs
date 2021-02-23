use std::{cell::RefCell, rc::Rc};

use vecmath::Vector2;

use crate::{joint::Joint, particle::Particle};

pub type DefaultWorld = World<Box<dyn Joint>>;

#[derive(Debug)]
pub struct World<T: Joint> {
    pub width: usize,
    pub height: usize,
    pub gravity: Vector2<f64>,

    pub particles: Vec<Rc<RefCell<Particle>>>,
    pub joints: Vec<T>,
}

impl<T: Joint> World<T> {
    pub fn new(width: usize, height: usize, gravity: Vector2<f64>) -> Self {
        Self {
            width,
            height,
            gravity,

            particles: Vec::new(),
            joints: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, p: Particle) -> Rc<RefCell<Particle>> {
        let p = Rc::new(RefCell::new(p));
        self.particles.push(Rc::clone(&p));
        Rc::clone(&p)
    }

    pub fn add_joint(&mut self, j: T) {
        self.joints.push(j);
    }

    pub fn update(&mut self, dt: f64) {
        // 速度
        for p in self.particles.iter_mut() {
            let mut p = p.borrow_mut();
            p.accelerate(self.gravity);
            p.update_velocity(dt);
        }

        // 衝突
        for i1 in 0..self.particles.len() {
            for i2 in (i1 + 1)..self.particles.len() {
                let p1 = &mut self.particles[i1].borrow_mut();
                let p2 = &mut self.particles[i2].borrow_mut();
                Self::collide(p1, p2);
            }
        }

        // ジョイント
        for j in self.joints.iter_mut() {
            j.update();
        }

        // 位置
        for p in self.particles.iter_mut() {
            let mut p = p.borrow_mut();
            p.update_position(dt);
            p.bounce(self.width, self.height);
        }
    }

    pub fn collide(p1: &mut Particle, p2: &mut Particle) {
        let dx = p1.position[0] - p2.position[0];
        let dy = p1.position[1] - p2.position[1];
        let distance = dx.hypot(dy);

        if distance < p1.size + p2.size {
            let n = vecmath::vec2_normalized_sub(p1.position, p2.position);

            let d1 = vecmath::vec2_dot(vecmath::vec2_sub(p2.velocity, p1.velocity), n) * 2.0;
            let d2 = -d1;

            let r1 = p1.mass / (p1.mass + p2.mass);
            let r2 = p2.mass / (p1.mass + p2.mass);

            let s1 = d1 * r1 * p1.material.restitution;
            let s2 = d2 * r2 * p2.material.restitution;

            p1.velocity = vecmath::vec2_add(vecmath::vec2_scale(n, s1), p1.velocity);
            p2.velocity = vecmath::vec2_add(vecmath::vec2_scale(n, s2), p2.velocity);
        }
    }
}
