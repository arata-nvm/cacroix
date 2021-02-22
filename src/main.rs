use std::{cell::RefCell, rc::Rc, thread, time::Duration};

use cacroix::{particle::Particle, world::World};

fn main() {
    let gravity = [0.0, -1.0];
    let mut world = World::new(100, 100, gravity);
    world.add_particle(Particle::new(50.0, 50.0));
    world.add_particle(Particle::new(50.0, 10.0));
    world.add_particle(Particle::new(25.0, 80.0));

    let mut tick = 0;
    println!("{}: {:?}", tick, world.particles);
    while tick < 15 {
        tick += 1;
        world.update();
        println!("{}: {:?}", tick, world.particles);
    }
}
