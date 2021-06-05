use crate::vecmath::vector2::{self, Vector2};

use super::body::Body;

// 2物体の相対速度を計算する
pub fn relative_velocty(b1: &Body, b2: &Body, point: Vector2) -> Vector2 {
    let mut relative_velocity = b1.velocity.sub(b2.velocity);
    relative_velocity.set_add(vector2::crossf(point.sub(b1.position), b1.angular_velocity));
    relative_velocity.set_sub(vector2::crossf(point.sub(b2.position), b2.angular_velocity));
    relative_velocity
}

// 法線方向の撃力を計算する
pub fn mass_normal(b1: &Body, b2: &Body, point: Vector2, normal: Vector2) -> f64 {
    let r1 = point.sub(b1.position);
    let r2 = point.sub(b2.position);
    let rn1 = vector2::dot(r1, normal);
    let rn2 = vector2::dot(r2, normal);
    return 1.0
        / ((b1.inv_mass + b2.inv_mass)
            + (b1.inv_i * (vector2::dot(r1, r1) - rn1 * rn1))
            + (b2.inv_i * (vector2::dot(r2, r2) - rn2 * rn2)));
}

// 接線方向の撃力を計算する
pub fn mass_tangent(b1: &Body, b2: &Body, point: Vector2, normal: Vector2) -> f64 {
    let r1 = point.sub(b1.position);
    let r2 = point.sub(b2.position);
    let tangent = vector2::crossf(normal, 1.0);
    let rt1 = vector2::dot(r1, tangent);
    let rt2 = vector2::dot(r2, tangent);
    return 1.0
        / ((b1.inv_mass + b2.inv_mass)
            + (b1.inv_i * (vector2::dot(r1, r1) - rt1 * rt1))
            + (b2.inv_i * (vector2::dot(r2, r2) - rt2 * rt2)));
}
