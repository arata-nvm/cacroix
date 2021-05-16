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

    pub radius: f64,
}

impl Body {
    pub fn new(radius: f64, density: f64, friction: f64) -> Self {
        let mut b = Self::default();

        b.radius = radius;
        b.friction = friction;

        b.mass = radius * radius * std::f64::consts::PI * density;
        b.inv_mass = 1.0 / b.mass;

        b.i = b.mass * radius * radius * 0.5;
        b.inv_i = 1.0 / b.i;

        b
    }

    pub fn apply_impulse(&mut self, impulse: Vector2, point: Vector2) {
        self.velocity.set_add(impulse.mul(self.inv_mass));
        self.angular_velocity += vector2::cross(point.sub(self.position), impulse) * self.inv_i;
    }
}
