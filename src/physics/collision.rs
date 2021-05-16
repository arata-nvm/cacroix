use id_arena::Arena;

use crate::vecmath::vector2;

use super::{
    body::{Body, BodyId},
    contact::Contact,
};

pub fn collide(bodies: &Arena<Body>, b1_id: BodyId, b2_id: BodyId) -> Option<Contact> {
    let b1 = bodies.get(b1_id).unwrap();
    let b2 = bodies.get(b2_id).unwrap();

    let diff = b2.position.sub(b1.position);
    if diff.len_sq() > (b1.radius + b2.radius) * (b1.radius + b2.radius) {
        return None;
    }

    let mut c = Contact::new(b1_id, b2_id);
    c.point = b1.position.add(b2.position).mul(0.5);
    c.normal = diff.normalized().neg();
    c.tangent = vector2::crossf(c.normal, 1.0);
    c.friction = f64::sqrt(b1.friction * b2.friction);
    Some(c)
}
