use std::time::{Duration, Instant};

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Timer {
    deadline: Instant,
    duration: Duration,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            deadline: Instant::now() + duration,
            duration,
        }
    }

    pub fn is_elapsed(&self) -> bool {
        Instant::now() >= self.deadline
    }

    pub fn reset(&mut self) {
        self.deadline = Instant::now() + self.duration
    }

    pub fn reset_with(&mut self, duration: Duration) {
        self.deadline = Instant::now() + duration;
        self.duration = duration;
    }
}

pub fn random_election_timeout() -> Duration {
    let mut rng = rand::rng();
    let millis = rng.random_range(150..=300);
    Duration::from_millis(millis)
}

pub fn heartbeat_interval() -> Duration {
    Duration::from_millis(50)
}
