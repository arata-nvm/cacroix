use std::{cell::RefCell, rc::Rc};

use crate::particle::Particle;

use super::Joint;

#[derive(Debug)]
pub struct SpringJoint {
    pub p1: Rc<RefCell<Particle>>,
    pub p2: Rc<RefCell<Particle>>,
    pub length: f64,
    pub strength: f64,
}

impl SpringJoint {
    pub fn new(
        p1: &Rc<RefCell<Particle>>,
        p2: &Rc<RefCell<Particle>>,
        length: f64,
        strength: f64,
    ) -> Self {
        Self {
            p1: Rc::clone(p1),
            p2: Rc::clone(p2),
            length,
            strength,
        }
    }
}

impl Joint for SpringJoint {
    fn update(&mut self) {
        let mut p1 = self.p1.borrow_mut();
        let mut p2 = self.p2.borrow_mut();

        let d = vecmath::vec2_sub(p1.position, p2.position);

        let distance = vecmath::vec2_len(d);
        let force = (self.length - distance) * self.strength;
        let f1 = force / p1.mass;
        let f2 = force / p2.mass;

        let dn = vecmath::vec2_normalized(d);
        p1.accelerate(vecmath::vec2_scale(dn, f1));
        p2.accelerate(vecmath::vec2_scale(dn, -f2));
    }

    fn particle1(&self) -> Rc<RefCell<Particle>> {
        Rc::clone(&self.p1)
    }

    fn particle2(&self) -> Rc<RefCell<Particle>> {
        Rc::clone(&self.p2)
    }
}
