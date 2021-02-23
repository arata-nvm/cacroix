use std::{cell::RefCell, rc::Rc};

use vecmath::Vector2;

use crate::{
    joint::distance::DistanceJoint,
    particle::{Material, Particle},
    world::DefaultWorld,
};

pub fn new_line(
    world: &mut DefaultWorld,
    p1: Vector2<f64>,
    p2: Vector2<f64>,
    size: f64,
    mass: f64,
    material: Material,
    strength: f64,
) -> Vec<Rc<RefCell<Particle>>> {
    let p = vec![
        world.add_particle(Particle::new(p1, size, mass, material)),
        world.add_particle(Particle::new(p2, size, mass, material)),
    ];
    let length = vecmath::vec2_len(vecmath::vec2_sub(p1, p2));
    world.add_joint(Box::new(DistanceJoint::new(&p[0], &p[1], length, strength)));
    return p;
}

pub fn new_rect(
    world: &mut DefaultWorld,
    p1: Vector2<f64>,
    p2: Vector2<f64>,
    size: f64,
    mass: f64,
    material: Material,
    strength: f64,
) -> Vec<Rc<RefCell<Particle>>> {
    let p = vec![
        world.add_particle(Particle::new([p1[0], p1[1]], size, mass, material)),
        world.add_particle(Particle::new([p2[0], p1[1]], size, mass, material)),
        world.add_particle(Particle::new([p1[0], p2[1]], size, mass, material)),
        world.add_particle(Particle::new([p2[0], p2[1]], size, mass, material)),
    ];
    for i1 in 0..4 {
        for i2 in (i1 + 1)..4 {
            let length = vecmath::vec2_len(vecmath::vec2_sub(
                p[i1].borrow().position,
                p[i2].borrow().position,
            ));
            world.add_joint(Box::new(DistanceJoint::new(
                &p[i1], &p[i2], length, strength,
            )));
        }
    }
    return p;
}

pub fn new_poly(
    world: &mut DefaultWorld,
    center: Vector2<f64>,
    radius: f64,
    num: usize,
    size: f64,
    mass: f64,
    material: Material,
    strength: f64,
) -> Vec<Rc<RefCell<Particle>>> {
    let mut p = Vec::new();
    for i in 0..num {
        let r = (2.0 * std::f64::consts::PI) * (i as f64 / num as f64);
        let px = r.cos() * radius;
        let py = r.sin() * radius;
        p.push(world.add_particle(Particle::new(
            vecmath::vec2_add(center, [px, py]),
            size,
            mass,
            material,
        )));
    }
    for i1 in 0..num {
        for i2 in (i1 + 1)..num {
            let length = vecmath::vec2_len(vecmath::vec2_sub(
                p[i1].borrow().position,
                p[i2].borrow().position,
            ));
            world.add_joint(Box::new(DistanceJoint::new(
                &p[i1], &p[i2], length, strength,
            )));
        }
    }
    return p;
}
