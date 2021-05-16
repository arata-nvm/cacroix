use id_arena::Arena;

use crate::vecmath::vector2::{self, Vector2};

use super::{
    body::{Body, BodyId},
    response,
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ContactKey(pub BodyId, pub BodyId);

#[derive(Debug, Clone)]
pub struct Contact {
    pub b1: BodyId,
    pub b2: BodyId,

    pub point: Vector2,
    pub normal: Vector2,
    pub tangent: Vector2,

    pub r1: Vector2,
    pub r2: Vector2,

    pub mass_normal: f64,
    pub mass_tangent: f64,

    pub normal_impulse: f64,
    pub tangent_impulse: f64,

    pub friction: f64,
    pub restitution: f64,
}

impl Contact {
    pub fn new(b1: BodyId, b2: BodyId) -> Self {
        Self {
            b1,
            b2,

            point: Default::default(),
            normal: Default::default(),
            tangent: Default::default(),

            mass_normal: Default::default(),
            mass_tangent: Default::default(),

            normal_impulse: Default::default(),
            tangent_impulse: Default::default(),

            friction: 0.0,
            restitution: 0.0,
        }
    }

    pub fn key(&self) -> ContactKey {
        ContactKey(self.b1, self.b2)
    }

    pub fn pre_step(&mut self, bodies: &Arena<Body>) {
        let b1 = bodies.get(self.b1).unwrap();
        let b2 = bodies.get(self.b2).unwrap();

        let relative_velocity = response::relative_velocty(b1, b2, self.point);
        let vrn = vector2::dot(relative_velocity, self.normal);
        if vrn < -1.0 {
            self.restitution =
                (-self.restitution * vector2::dot(self.normal, relative_velocity)).max(0.0);
        }

        self.mass_normal = response::mass_normal(b1, b2, self.point, self.normal);
        self.mass_tangent = response::mass_tangent(b1, b2, self.point, self.normal);
    }

    pub fn apply_impulse(&mut self, bodies: &mut Arena<Body>) {
        {
            let b1 = bodies.get_mut(self.b1).unwrap();
            b1.apply_impulse(self.normal.mul(self.mass_normal), self.point);

            let b2 = bodies.get_mut(self.b2).unwrap();
            b2.apply_impulse(self.normal.mul(self.mass_normal).neg(), self.point);
        }

        let max_mass_tangent = self.mass_normal * self.friction;
        let mass_tangent = self
            .mass_tangent
            .min(max_mass_tangent)
            .max(-max_mass_tangent);

        {
            let b1 = bodies.get_mut(self.b1).unwrap();
            b1.apply_impulse(self.tangent.mul(mass_tangent), self.point);

            let b2 = bodies.get_mut(self.b2).unwrap();
            b2.apply_impulse(self.tangent.mul(mass_tangent).neg(), self.point);
        }
    }

    pub fn apply_position_impulse(&mut self, bodies: &mut Arena<Body>) {
        let b1 = bodies.get(self.b1).unwrap();
        let b2 = bodies.get(self.b2).unwrap();

        let rb1 = b1.position.add(self.r);
        let rb2 = b1.position.add(b2.r1);
    }
}
