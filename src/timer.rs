use std::time::Duration;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Timer {
    duration: Duration,
    elapsed: Duration,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            elapsed: Duration::from_secs(0),
        }
    }

    pub fn tick(&mut self, delta: Duration) -> bool {
        self.elapsed += delta;
        if self.elapsed >= self.duration {
            let times = self.elapsed.as_nanos() / self.duration.as_nanos();
            self.elapsed -= times as u32 * self.duration;
            true
        } else {
            false
        }
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }
}
