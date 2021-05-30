use id_arena::Id;

use crate::vecmath::vector2::{self, Vector2};

pub type BodyId = Id<Body>;

#[derive(Debug, Default)]
pub struct Body {
    pub position: Vector2,
    pub velocity: Vector2,
    pub force: Vector2,

    pub rotation: f64,
    pub angular_velocity: f64,
    pub torque: f64,

    pub mass: f64,
    pub inv_mass: f64,

    pub i: f64,
    pub inv_i: f64,

    pub friction: f64,
    pub restitution: f64,

    pub radius: f64,

    pub is_static: bool,
}

impl Body {
    pub fn new(radius: f64, density: f64, friction: f64, restitution: f64) -> Self {
        let mut b = Self::default();

        b.radius = radius;
        b.friction = friction;
        b.restitution = restitution;

        b.mass = radius * radius * std::f64::consts::PI * density;
        b.inv_mass = 1.0 / b.mass;

        b.i = b.mass * radius * radius * 0.5;
        b.inv_i = 1.0 / b.i;

        b
    }

    pub fn set_static(&mut self) {
        self.is_static = true;
        self.mass = f64::MAX;
        self.inv_mass = 0.0;
        self.i = f64::MAX;
        self.inv_i = 0.0;
    }

    pub fn apply_impulse(&mut self, impulse: Vector2, point: Vector2) {
        if self.is_static {
            return;
        }

        self.velocity.set_add(impulse.mul(self.inv_mass));
        self.angular_velocity += vector2::cross(point.sub(self.position), impulse) * self.inv_i;
    }

    pub fn apply_position_impulse(&mut self, impulse: Vector2, point: Vector2) {
        if self.is_static {
            return;
        }

        self.position.set_add(impulse.mul(self.inv_mass));
        self.rotation += vector2::cross(point.sub(self.position), impulse) * self.inv_i;
    }
}
