use std::{cell::RefCell, rc::Rc};

use crate::dynamic::particle::Particle;

use super::Joint;

#[derive(Debug)]
pub struct MortorJoint {
    pub p1: Rc<RefCell<Particle>>,
    pub p2: Rc<RefCell<Particle>>,
    pub length: f64,
    pub power: f64,
}

impl MortorJoint {
    pub fn new(
        p1: &Rc<RefCell<Particle>>,
        p2: &Rc<RefCell<Particle>>,
        length: f64,
        power: f64,
    ) -> Self {
        Self {
            p1: Rc::clone(p1),
            p2: Rc::clone(p2),
            length,
            power,
        }
    }
}

impl Joint for MortorJoint {
    fn update(&mut self) {
        let mut p1 = self.p1.borrow_mut();
        let mut p2 = self.p2.borrow_mut();

        let n = vecmath::vec2_normalized_sub(p2.position, p1.position);
        let rel_vel = vecmath::vec2_dot(vecmath::vec2_sub(p2.velocity, p1.velocity), n);

        let distance = vecmath::vec2_len(vecmath::vec2_sub(p1.position, p2.position));
        let diff = distance - self.length;

        let v_dis = vecmath::vec2_scale(n, (rel_vel + diff) * 0.5);
        let v_rot = vecmath::vec2_scale([n[1], -n[0]], self.power);

        let v = vecmath::vec2_add(v_dis, v_rot);

        p1.velocity = vecmath::vec2_add(p1.velocity, v);
        p2.velocity = vecmath::vec2_sub(p2.velocity, v);
    }

    fn particle1(&self) -> Rc<RefCell<Particle>> {
        Rc::clone(&self.p1)
    }

    fn particle2(&self) -> Rc<RefCell<Particle>> {
        Rc::clone(&self.p2)
    }
}
