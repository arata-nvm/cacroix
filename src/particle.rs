use vecmath::Vector2;

#[derive(Debug)]
pub struct Particle {
    pub velocity: Vector2<f64>,
    pub position: Vector2<f64>,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            velocity: Vector2::default(),
            position: [x, y],
        }
    }

    pub fn accelerate(&mut self, v: Vector2<f64>) {
        self.velocity = vecmath::vec2_add(self.velocity, v);
    }

    pub fn move_(&mut self) {
        self.position = vecmath::vec2_add(self.position, self.velocity);
    }
}
