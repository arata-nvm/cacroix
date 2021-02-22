use cacroix::{
    particle::{Material, Particle},
    spring::Spring,
    world::World,
};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    world: World,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let scale = 2.0;

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
            .springs
            .iter()
            .map(|s| {
                let p1 = s.p1.borrow();
                let p2 = s.p2.borrow();
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

            let trans = c.transform.scale(scale, scale);

            for r in rects {
                ellipse(RED, r, trans, gl);
            }

            for l in lines {
                line(RED, 0.3, l, trans, gl);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.world.update();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        world: init_world(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

fn init_world() -> World {
    let gravity = [0.0, 0.02];
    let mut world = World::new(100, 100, gravity);
    new_square(&mut world);
    return world;
}

fn new_square(world: &mut World) {
    let m = Material {
        linear_damping: 0.999,
        restitution: 0.75,
    };

    let base = [10.0, 10.0];
    let size = 50.0;

    let p = vec![
        world.add_particle(Particle::new(base, 1.0, 200.0, m)),
        world.add_particle(Particle::new(
            vecmath::vec2_add(base, [size, 0.0]),
            1.0,
            200.0,
            m,
        )),
        world.add_particle(Particle::new(
            vecmath::vec2_add(base, [size, size]),
            1.0,
            200.0,
            m,
        )),
        world.add_particle(Particle::new(
            vecmath::vec2_add(base, [size, size]),
            1.0,
            200.0,
            m,
        )),
    ];

    for i1 in 0..p.len() {
        for i2 in (i1 + 1)..p.len() {
            world.add_spring(Spring::new(&p[i1], &p[i2], size, 0.75));
        }
    }
}
