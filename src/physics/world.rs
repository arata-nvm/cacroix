use std::collections::HashMap;

use id_arena::Arena;

use crate::vecmath::vector2::Vector2;

use super::{
    body::{Body, BodyId},
    collision,
    contact::{Contact, ContactKey},
};

#[derive(Debug)]
pub struct World {
    pub bodies: Arena<Body>,
    pub contacts: HashMap<ContactKey, Contact>,

    pub gravity: Vector2,
    pub iterations: usize,
}

impl World {
    pub fn new(gravity: Vector2, iterations: usize) -> Self {
        Self {
            bodies: Arena::new(),
            contacts: HashMap::new(),
            gravity,
            iterations,
        }
    }

    pub fn add(&mut self, body: Body) {
        self.bodies.alloc(body);
    }

    // 時間を進める
    pub fn step(&mut self, dt: f64) {
        // 加速度を反映する
        for (_, body) in &mut self.bodies {
            if !body.is_static {
                body.velocity.set_add(self.gravity.mul(dt));
                body.velocity.set_add(body.force.mul(body.inv_mass).mul(dt));

                body.angular_velocity += dt * body.inv_i * body.torque;
            }
        }

        // 衝突を検出する
        let body_ids: Vec<BodyId> = self.bodies.iter().map(|(id, _)| id).collect();
        for i in 0..self.bodies.len() {
            for j in 0..i {
                let b1 = body_ids[i];
                let b2 = body_ids[j];

                if let Some(c) = collision::collide(&self.bodies, b1, b2) {
                    self.contacts
                        .entry(c.key())
                        .and_modify(|v| *v = v.merge(c.clone()))
                        .or_insert(c);
                } else {
                    self.contacts.remove(&ContactKey(b1, b2));
                }
            }
        }

        // 拘束の準備
        for c in &mut self.contacts.values_mut() {
            c.pre_step(&mut self.bodies);
        }

        // 拘束を解決する
        for _ in 0..self.iterations {
            for c in &mut self.contacts.values_mut() {
                c.apply_impulse(&mut self.bodies);
            }
        }

        // 速度を反映する
        for (_, body) in &mut self.bodies {
            if !body.is_static {
                body.position.set_add(body.velocity.mul(dt));
                body.rotation += body.angular_velocity * dt;

                body.force.set_zero();
                body.torque = 0.0;
            }
        }

        // めり込みを修正する
        for _ in 0..self.iterations {
            for c in &mut self.contacts.values_mut() {
                c.apply_position_impulse(&mut self.bodies);
            }
        }

        //
        self.loop_edge();
    }

    pub fn loop_edge(&mut self) {
        let size = 800.0;
        for (_, b) in &mut self.bodies {
            if b.position[0] <= 0.0 {
                b.position[0] = size - 1.0;
            }
            if b.position[1] <= 0.0 {
                b.position[1] = size - 1.0;
            }

            if b.position[0] >= size {
                b.position[0] = 0.0;
            }
            if b.position[1] >= size {
                b.position[1] = 0.0;
            }
        }
    }
}
