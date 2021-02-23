use std::{cell::RefCell, rc::Rc};

use super::particle::Particle;

pub mod distance;
pub mod spring;

pub trait Joint {
    fn update(&mut self);

    fn particle1(&self) -> Rc<RefCell<Particle>>;

    fn particle2(&self) -> Rc<RefCell<Particle>>;
}

impl Joint for Box<dyn Joint> {
    fn update(&mut self) {
        self.as_mut().update();
    }

    fn particle1(&self) -> Rc<RefCell<Particle>> {
        self.as_ref().particle1()
    }

    fn particle2(&self) -> Rc<RefCell<Particle>> {
        self.as_ref().particle2()
    }
}
