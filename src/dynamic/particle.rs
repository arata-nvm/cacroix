use vecmath::Vector2;

#[derive(Debug)]
pub struct Particle {
    pub acceleration: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub position: Vector2<f64>,

    pub size: f64,
    pub mass: f64,
    pub typ: Type,
    pub material: Material,
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    // 反発
    pub restitution: f64,

    // 線形減衰
    pub linear_damping: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Dynamic,
    Static,
}

pub const MAX_VELOCITY: f64 = 1000.0;

impl Particle {
    pub fn new(position: Vector2<f64>, size: f64, mass: f64, material: Material) -> Self {
        Self {
            acceleration: [0.0, 0.0],
            velocity: [0.0, 0.0],
            position,
            size,
            mass,
            typ: Type::Dynamic,
            material,
        }
    }

    pub fn accelerate(&mut self, v: Vector2<f64>) {
        self.acceleration = vecmath::vec2_add(self.acceleration, v);
    }

    pub fn bounce(&mut self, width: usize, height: usize) {
        let (width, height) = (width as f64, height as f64);

        // 左面
        if self.position[0] < self.size {
            self.position[0] = 2.0 * self.size - self.position[0];
            self.velocity[0] = -self.velocity[0] * self.material.restitution;
        }
        // 右面
        if self.position[0] > width - self.size {
            self.position[0] = 2.0 * (width - self.size) - self.position[0];
            self.velocity[0] = -self.velocity[0] * self.material.restitution;
        }

        // 上面
        if self.position[1] < self.size {
            self.position[1] = 2.0 * self.size - self.position[1];
            self.velocity[1] = -self.velocity[1] * self.material.restitution;
        }
        // 下面
        if self.position[1] > height - self.size {
            self.position[1] = 2.0 * (height - self.size) - self.position[1];
            self.velocity[1] = -self.velocity[1] * self.material.restitution;
        }
    }

    pub fn update_velocity(&mut self, dt: f64) {
        if self.typ == Type::Static {
            self.acceleration = [0.0, 0.0];
            self.velocity = [0.0, 0.0];
            return;
        }

        let acceleration = vecmath::vec2_scale(self.acceleration, dt);
        self.velocity = vecmath::vec2_add(self.velocity, acceleration);
    }

    pub fn update_position(&mut self, dt: f64) {
        if self.typ == Type::Static {
            return;
        }

        let cur_len = vecmath::vec2_square_len(self.velocity);
        let max_len = MAX_VELOCITY * MAX_VELOCITY;
        if cur_len > max_len {
            self.velocity = vecmath::vec2_scale(self.velocity, max_len / cur_len);
        }

        let velocity = vecmath::vec2_scale(self.velocity, dt);
        self.position = vecmath::vec2_add(self.position, velocity);
        self.acceleration = [0.0, 0.0];
    }
}
