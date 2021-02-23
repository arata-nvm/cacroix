use cacroix::{
    common::shape,
    dynamic::{
        joint::{distance::DistanceJoint, mortor::MortorJoint, Joint},
        particle::{self, Material, Particle},
        world::{DefaultWorld, World},
    },
};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;
use piston::{
    event_loop::{EventSettings, Events},
    keyboard, PressEvent,
};
use piston::{
    input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent},
    Button,
};

pub struct App {
    gl: GlGraphics,
    world: DefaultWorld,
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
        self.world.update(args.dt * 4.0);
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

        if let Some(Button::Keyboard(keyboard::Key::Space)) = e.press_args() {
            app.world = init_world();
        }
    }
}

fn init_world() -> DefaultWorld {
    return init_world2();
}

fn init_world1() -> DefaultWorld {
    let gravity = [0.0, 9.8];
    let mut world = World::new(800, 600, gravity);

    let p1 = [100.0, 100.0];
    let p2 = [200.0, 200.0];
    let size = 2.0;
    let mass = 20.0;
    let material = Material {
        linear_damping: 0.999,
        restitution: 0.25,
    };
    let strength = 1.5;
    shape::new_rect(&mut world, p1, p2, size, mass, material, strength);

    let center = [300.0, 300.0];
    let radius = 100.0;
    let num = 10;
    shape::new_poly(
        &mut world, center, radius, num, size, mass, material, strength,
    );

    return world;
}

fn init_world2() -> DefaultWorld {
    let gravity = [0.0, 9.8];
    let mut world = World::new(800, 600, gravity);

    let m = Material {
        linear_damping: 0.999,
        restitution: 0.1,
    };

    for i in 0..200 {
        world.add_particle(Particle::new(
            [400.0 + 0.1 * i as f64, 100.0 + 50.0 * i as f64],
            10.0,
            1.0,
            m,
        ));
    }

    return world;
}

fn init_world3() -> DefaultWorld {
    let gravity = [0.0, 9.8];
    let mut world: DefaultWorld = World::new(800, 600, gravity);

    let m = Material {
        linear_damping: 0.999,
        restitution: 0.1,
    };

    let p1 = world.add_particle(Particle::new([400.0, 300.0], 10.0, 10.0, m));
    let p2 = world.add_particle(Particle::new([400.0, 400.0], 10.0, 10.0, m));
    world.add_joint(Box::new(MortorJoint::new(&p1, &p2, 100.0, 1.0)));

    for i in 0..10 {
        world.add_particle(Particle::new([80.0 * i as f64, 500.0], 10.0, 10.0, m));
    }

    p1.borrow_mut().typ = particle::Type::Static;

    return world;
}

fn init_world4() -> DefaultWorld {
    let gravity = [0.0, 9.8];
    let mut world: DefaultWorld = World::new(800, 600, gravity);

    let m = Material {
        linear_damping: 0.999,
        restitution: 0.1,
    };

    let mut p = Vec::new();
    for i in 0..10 {
        p.push(world.add_particle(Particle::new(
            [80.0 * i as f64 + 40.0, 100.0],
            10.0,
            10.0,
            m,
        )));
    }

    p.first_mut().unwrap().borrow_mut().typ = particle::Type::Static;
    p.last_mut().unwrap().borrow_mut().typ = particle::Type::Static;

    for i in 0..9 {
        world.add_joint(Box::new(DistanceJoint::new(&p[i], &p[i + 1], 80.0, 2.0)));
    }
    return world;
}
