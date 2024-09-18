use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct TickCounter {
    timestamps: VecDeque<Instant>,
}

impl TickCounter {
    pub fn new() -> Self {
        TickCounter {
            timestamps: VecDeque::new(),
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.timestamps.push_back(now);
        self.cleanup(now);
    }

    pub fn count_last_second(&self) -> usize {
        self.timestamps.len()
    }

    fn cleanup(&mut self, now: Instant) {
        while let Some(&oldest) = self.timestamps.front() {
            if now.duration_since(oldest) > Duration::from_secs(1) {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }
    }
}
