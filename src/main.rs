use cacroix::{
    particle::{Material, Particle},
    world::World,
};

fn main() {
    let gravity = [0.0, 1.0];
    let size = 1.0;
    let m = Material {
        linear_damping: 0.999,
        restitution: 0.75,
    };

    let mut world = World::new(100, 100, gravity);
    world.add_particle(Particle::new(50.0, 50.0, size, m));
    world.add_particle(Particle::new(50.0, 10.0, size, m));
    world.add_particle(Particle::new(25.0, 80.0, size, m));

    let mut tick = 0;
    println!("{}: {:?}", tick, world.particles);
    while tick < 15 {
        tick += 1;
        world.update();
        println!("{}: {}", tick, dump_particles(&world));
    }
}

fn dump_particles(world: &World) -> String {
    let positions = world.particles.iter().map(|p| p.position);
    let velocities = world.particles.iter().map(|p| p.velocity);
    return format!("{:.02?}", positions.zip(velocities).collect::<Vec<_>>());
}
