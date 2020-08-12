const ALPHA: f64 = 0.05;

#[derive(Clone, Copy)]
pub struct TransferRate {
    prev_bytes: u64,
    prev_speed: f64,
}

impl TransferRate {
    pub fn new() -> Self {
        Self {
            prev_bytes: 0,
            prev_speed: 0.,
        }
    }

    pub fn update(&mut self, bytes: u64, delta_time: f64) -> f64 {
        let current_speed = (bytes - self.prev_bytes) as f64 / delta_time;
        if self.prev_speed == 0. {
            self.prev_speed = current_speed;
        } else {
            self.prev_speed = (1. - ALPHA) * self.prev_speed + ALPHA * current_speed;
        }
        self.prev_bytes = bytes;
        self.prev_speed
    }
}

