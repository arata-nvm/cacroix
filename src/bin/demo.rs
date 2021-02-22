use cacroix::{
    joint::{distance::DistanceJoint, Joint},
    particle::{self, Material, Particle},
    world::World,
};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    world: World<Box<dyn Joint>>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let rects: Vec<types::Rectangle> = self
            .world
            .particles
            .iter()
            .map(|p| {
                let p = p.borrow();
                rectangle::centered_square(p.position[0], p.position[1], p.size)
            })
            .collect();

        let lines: Vec<types::Line> = self
            .world
            .joints
            .iter()
            .map(|s| {
                let p1 = s.particle1();
                let p1 = p1.borrow();
                let p2 = s.particle2();
                let p2 = p2.borrow();
                [
                    p1.position[0],
                    p1.position[1],
                    p2.position[0],
                    p2.position[1],
                ]
            })
            .collect();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let trans = c.transform;

            for r in rects {
                ellipse(RED, r, trans, gl);
            }

            for l in lines {
                line(RED, 0.9, l, trans, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.world.update(args.dt);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        world: init_world(),
    };

    let mut events = Events::new(EventSettings {
        max_fps: 60,
        ..EventSettings::default()
    });
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

const M: Material = Material {
    linear_damping: 0.999,
    restitution: 0.25,
};

fn init_world() -> World<Box<dyn Joint>> {
    let gravity = [0.0, 9.8];
    let mut world = World::new(800, 600, gravity);
    // new_line(&mut world);
    new_triangle(&mut world);
    // new_square(&mut world);
    return world;
}

fn new_line(world: &mut World<Box<dyn Joint>>) {
    let mass = 150.0;

    let p1 = world.add_particle(Particle::new([200.1, 10.0], 2.0, mass, M));
    let p2 = world.add_particle(Particle::new([200.0, 110.0], 2.0, mass, M));
    // p2.borrow_mut().typ = particle::Type::Static;

    world.add_joint(Box::new(DistanceJoint::new(&p1, &p2, 100.0, 0.0)));
}

fn new_triangle(world: &mut World<Box<dyn Joint>>) {
    let mass = 150.0;

    let p1 = world.add_particle(Particle::new([400.0, 10.0], 2.0, mass, M));
    let p2 = world.add_particle(Particle::new([450.0, 110.0], 2.0, mass, M));
    let p3 = world.add_particle(Particle::new([350.0, 110.0], 2.0, mass, M));
    // p3.borrow_mut().typ = particle::Type::Static;
    world.add_joint(Box::new(DistanceJoint::new(&p1, &p2, 112.0, 0.1)));
    world.add_joint(Box::new(DistanceJoint::new(&p2, &p3, 100.0, 0.1)));
    world.add_joint(Box::new(DistanceJoint::new(&p3, &p1, 112.0, 0.1)));
}

fn new_square(world: &mut World<Box<dyn Joint>>) {
    let base = [10.0, 10.0];
    let size = 100.0;
    let mass = 150.0;

    let p = vec![
        world.add_particle(Particle::new(base, 1.0, mass, M)),
        world.add_particle(Particle::new(
            vecmath::vec2_add(base, [size, 0.0]),
            1.0,
            mass,
            M,
        )),
        world.add_particle(Particle::new(
            vecmath::vec2_add(base, [size, size]),
            1.0,
            mass,
            M,
        )),
        world.add_particle(Particle::new(
            vecmath::vec2_add(base, [0.0, size]),
            1.0,
            mass,
            M,
        )),
    ];

    for i1 in 0..p.len() {
        for i2 in (i1 + 1)..p.len() {
            let size = if (i1 == 0 && i2 == 2) || (i1 == 1 && i2 == 3) {
                (size * size + size * size).sqrt()
            } else {
                size
            };
            world.add_joint(Box::new(DistanceJoint::new(&p[i1], &p[i2], size, 0.1)));
        }
    }
}
