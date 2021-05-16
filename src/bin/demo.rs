use std::time::Instant;

use cacroix::{
    physics::{body::Body, world::World},
    vecmath::vector2::Vector2,
};
use conrod::{
    backend::glium::glium, glium::Surface, widget, widget::Widget, widget_ids, Colorable,
    Positionable,
};

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Demo")
        .with_dimensions(800, 800);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([800.0, 800.0]).build();

    widget_ids! {struct Ids {circles[], lines[]}};
    let mut ids = Ids::new(ui.widget_id_generator());

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let mut world = new_world();
    ids.circles
        .resize(world.bodies.len(), &mut ui.widget_id_generator());
    ids.lines
        .resize(world.bodies.len(), &mut ui.widget_id_generator());

    let mut events = Vec::new();

    let mut last_update = Instant::now();
    let sixteen_ms = std::time::Duration::from_millis(16);

    'render: loop {
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        events.clear();
        events_loop.poll_events(|event| events.push(event));

        for event in events.drain(..) {
            match event.clone() {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'render,
                    _ => {}
                },
                _ => {}
            }
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();

            let mut ui = ui.set_widgets();
            for (i, (_, body)) in world.bodies.iter().enumerate() {
                let p1 = body.position;
                let p2 = p1.add(Vector2::new(
                    f64::cos(body.rotation) * body.radius,
                    f64::sin(body.rotation) * body.radius,
                ));

                widget::Circle::outline(body.radius)
                    .x(p1[0])
                    .y(p1[1])
                    .color(conrod::color::RED)
                    .set(ids.circles[i], &mut ui);

                widget::Line::new([p1[0], p1[1]], [p2[0], p2[1]])
                    .color(conrod::color::RED)
                    .set(ids.lines[i], &mut ui)
            }
            world.step((1.0 / 60.0) * 4.0);
        }

        last_update = Instant::now();
    }
}

fn new_world() -> World {
    let g = Vector2::new(0.0, 0.0);
    let mut world = World::new(g, 10);

    let mut b1 = Body::new(20.0, 1.0, 0.2);
    b1.position.set(-100.0, 0.0);
    b1.velocity.set(10.0, 0.0);
    world.add(b1);

    let mut b2 = Body::new(20.0, 1.0, 0.2);
    b2.position.set(0.0, -300.0);
    b2.velocity.set(0.0, 50.0);
    b2.angular_velocity = 100.0;
    world.add(b2);

    world
}
