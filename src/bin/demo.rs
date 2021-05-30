use std::{thread, time};

use cacroix::{
    physics::{body::Body, world::World},
    vecmath::vector2::Vector2,
};

use tetra::graphics::{self, Color};
use tetra::graphics::{
    mesh::{Mesh, ShapeStyle},
    DrawParams,
};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

struct GameState {
    world: World,
}

impl GameState {
    fn new(_ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self { world: new_world() })
    }
}

fn new_world() -> World {
    let g = Vector2::new(0.0, 9.8);
    let mut world = World::new(g, 10);

    // let mut b11 = Body::new(20.0, 1.0, 0.2);
    // b11.position.set(370.0, 20.0);
    // b11.velocity.set(0.0, 0.0);
    // world.add(b11);

    for i in 0..10 {
        let mut b = Body::new(20.0, 1.0, 0.2, 0.2);
        b.position.set(400.001, 20.0 + 50.0 * i as f64);
        b.velocity.set(0.0, 0.0);
        world.add(b);
    }

    let mut b2 = Body::new(50.0, 1.0, 0.2, 0.2);
    b2.position.set(400.0, 700.0);
    b2.velocity.set(0.0, 00.0);
    b2.set_static();
    world.add(b2);

    world
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context) -> tetra::Result {
        self.world.step((1.0 / 60.0) * 4.0);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

        for (_, body) in &self.world.bodies {
            let p = Vec2::new(body.position[0] as f32, body.position[1] as f32);
            let d = Vec2::new(
                f32::cos(body.rotation as f32),
                f32::sin(body.rotation as f32),
            ) * body.radius as f32;

            let circle = Mesh::circle(ctx, ShapeStyle::Stroke(1.0), p, body.radius as f32)?;
            let line = Mesh::polyline(ctx, 1.0, &[p, p + d])?;
            circle.draw(ctx, DrawParams::new());
            line.draw(ctx, DrawParams::new());
        }

        for (_, c) in &self.world.contacts {
            let p = Vec2::new(c.point[0] as f32, c.point[1] as f32);
            let circle = Mesh::circle(ctx, ShapeStyle::Stroke(1.0), p, 2.0)?;
            circle.draw(ctx, DrawParams::new());
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Demo", 800, 800)
        .build()?
        .run(GameState::new)
}
