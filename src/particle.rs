use vecmath::Vector2;

#[derive(Debug)]
pub struct Particle {
    pub velocity: Vector2<f64>,
    pub position: Vector2<f64>,
    pub size: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            velocity: [0.0, 0.0],
            position: [x, y],
            size: 1.0,
        }
    }

    pub fn accelerate(&mut self, v: Vector2<f64>) {
        self.velocity = vecmath::vec2_add(self.velocity, v);
    }

    pub fn bounce(&mut self, width: usize, height: usize) {
        let (width, height) = (width as f64, height as f64);

        // 左面
        if self.position[0] < self.size {
            self.position[0] = self.size - self.position[0];
            self.velocity[0] = -self.velocity[0];
        }
        // 右面
        if self.position[0] > width - self.size {
            self.position[0] = 2.0 * (width - self.size) - self.position[0];
            self.velocity[0] = -self.velocity[0];
        }

        // 上面
        if self.position[1] < self.size {
            self.position[1] = self.size - self.position[1];
            self.velocity[1] = -self.velocity[1];
        }
        // 下面
        if self.position[1] > height - self.size {
            self.position[1] = 2.0 * (height - self.size) - self.position[1];
            self.velocity[1] = -self.velocity[1];
        }
    }

    pub fn move_(&mut self) {
        self.position = vecmath::vec2_add(self.position, self.velocity);
    }
}
