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
    // 接触している物体
    pub b1: BodyId,
    pub b2: BodyId,

    // 接触位置
    pub point: Vector2,
    // 法線ベクトル
    pub normal: Vector2,
    // 接戦ベクトル
    pub tangent: Vector2,

    // 接触点の相対位置
    pub r1: Vector2,
    pub r2: Vector2,

    //　相対速度
    pub relative_velocity: Vector2,

    // 法線方向の適正質量
    pub mass_normal: f64,
    // 接線方向の適正質量
    pub mass_tangent: f64,

    // 法線方向の力積
    pub normal_impulse: f64,
    // 接線方向の力積
    pub tangent_impulse: f64,
    pub moving_impulse: f64,

    // 摩擦係数
    pub friction: f64,
    // 反発係数
    pub restitution: f64,

    // めり込み距離
    pub overlap: f64,
}

impl Contact {
    pub fn new(b1: BodyId, b2: BodyId) -> Self {
        Self {
            b1,
            b2,

            point: Default::default(),
            normal: Default::default(),
            tangent: Default::default(),

            r1: Default::default(),
            r2: Default::default(),

            relative_velocity: Default::default(),

            mass_normal: Default::default(),
            mass_tangent: Default::default(),

            normal_impulse: Default::default(),
            tangent_impulse: Default::default(),
            moving_impulse: Default::default(),

            friction: Default::default(),
            restitution: Default::default(),

            overlap: Default::default(),
        }
    }

    pub fn key(&self) -> ContactKey {
        ContactKey(self.b1, self.b2)
    }

    pub fn merge(&self, mut new_c: Contact) -> Contact {
        new_c.normal_impulse = self.normal_impulse;
        new_c.tangent_impulse = self.tangent_impulse;
        new_c.restitution = 0.0;
        new_c
    }

    pub fn pre_step(&mut self, bodies: &mut Arena<Body>) {
        let b1 = bodies.get(self.b1).unwrap();
        let b2 = bodies.get(self.b2).unwrap();

        self.friction = f64::sqrt(b1.friction * b2.friction); // 摩擦係数
        self.restitution = f64::sqrt(b1.restitution * b2.restitution); // 反発係数

        self.relative_velocity = response::relative_velocty(b1, b2, self.point); // 相対速度
        let rvn = vector2::dot(self.relative_velocity, self.normal); // 法線方向の相対速度
        if rvn < -1.0 {
            //
            self.restitution = (self.restitution * -rvn).max(0.0);
        }

        self.mass_normal = response::mass_normal(b1, b2, self.point, self.normal);
        self.mass_tangent = response::mass_tangent(b1, b2, self.point, self.normal);
        self.r1 = self.point.sub(b1.position).rot_rev(b1.rotation);
        self.r2 = self.point.sub(b2.position).rot_rev(b2.rotation);
        self.moving_impulse = 0.0;
    }

    pub fn apply_impulse(&mut self, bodies: &mut Arena<Body>) {
        let b1 = bodies.get(self.b1).unwrap();
        let b2 = bodies.get(self.b2).unwrap();

        self.relative_velocity = response::relative_velocty(b1, b2, self.point); // 相対速度
        let rvn = vector2::dot(self.relative_velocity, self.normal); // 法線方向の相対速度
        let new_normal_impulse = -self.mass_normal * (rvn - self.restitution);
        let new_normal_impulse = (self.normal_impulse + new_normal_impulse).max(0.0);
        let normal_impulse = new_normal_impulse - self.normal_impulse; // 前回との解の差

        // それぞれの物体に法線方向の撃力を適用する
        {
            let b1 = bodies.get_mut(self.b1).unwrap();
            b1.apply_impulse(self.normal.mul(normal_impulse), self.point);

            let b2 = bodies.get_mut(self.b2).unwrap();
            b2.apply_impulse(self.normal.mul(normal_impulse).neg(), self.point);
        }

        let rvt = vector2::dot(self.tangent, self.relative_velocity); // 接線方向の相対速度
        let max_tangent_impulse = self.normal_impulse * self.friction;
        let new_tangent_impulse = self.normal_impulse * -rvt;
        let new_tangent_impulse = (self.tangent_impulse + new_tangent_impulse)
            .max(-max_tangent_impulse)
            .min(max_tangent_impulse);
        let tangent_impulse = new_tangent_impulse - self.tangent_impulse; // 前回との解の差

        // それぞれの物体に接線方向の撃力を適用する
        {
            let b1 = bodies.get_mut(self.b1).unwrap();
            b1.apply_impulse(self.tangent.mul(tangent_impulse), self.point);

            let b2 = bodies.get_mut(self.b2).unwrap();
            b2.apply_impulse(self.tangent.mul(tangent_impulse).neg(), self.point);
        }

        self.normal_impulse = new_normal_impulse;
        self.tangent_impulse = new_tangent_impulse;
    }

    pub fn apply_position_impulse(&mut self, bodies: &mut Arena<Body>) {
        let b1 = bodies.get(self.b1).unwrap();
        let b2 = bodies.get(self.b2).unwrap();

        let rb1 = b1.position.add(self.r1.rot(b1.rotation));
        let rb2 = b2.position.add(self.r2.rot(b2.rotation));

        let dist = -self.overlap - vector2::dot(rb2.sub(rb1), self.normal); // 重なっている距離
        let dist = dist + 0.005; // 余裕をもたせる
        let impulse = -self.mass_normal * 0.2 * dist.min(0.0); // 補正のための撃力を計算
        let temp = self.moving_impulse;
        self.moving_impulse = (self.moving_impulse + impulse).max(0.0);
        let impulse2 = self.moving_impulse - temp;

        {
            let b1 = bodies.get_mut(self.b1).unwrap();
            b1.apply_position_impulse(self.normal.mul(impulse2), rb1);

            let b2 = bodies.get_mut(self.b2).unwrap();
            b2.apply_position_impulse(self.normal.mul(impulse2).neg(), rb2);
        }
    }
}
