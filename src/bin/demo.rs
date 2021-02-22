use cacroix::{
    particle::{Material, Particle},
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
            .map(|p| rectangle::centered_square(p.position[0], p.position[1], p.size))
            .collect();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let trans = c.transform.scale(scale, scale);

            for rect in rects {
                ellipse(RED, rect, trans, gl);
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
    let size = 2.0;
    let m = Material {
        linear_damping: 0.999,
        restitution: 0.75,
    };

    let mut world = World::new(100, 100, gravity);

    let mut p1 = Particle::new(50.0, 50.0, size, 200.0, m);
    p1.accelerate([-0.1, 0.5]);

    let mut p2 = Particle::new(50.0, 10.0, size, 100.0, m);
    p2.accelerate([0.2, 0.5]);

    let mut p3 = Particle::new(25.0, 80.0, size, 200.0, m);
    p3.accelerate([0.1, 0.5]);

    world.add_particle(p1);
    world.add_particle(p2);
    world.add_particle(p3);

    return world;
}
