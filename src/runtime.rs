use stopwatch::Stopwatch;
use std::{collections::HashMap, time::{Duration, Instant}};

pub const WINDOW_SEC: u64 = 1;

pub struct RuntimeAnalysis {
    pub names: Vec<String>,
    pub counts: Vec<usize>,
    pub totals: Vec<u128>,
    pub averages: Vec<f32>,
}

pub struct Runtime {
    timers: HashMap<String, Vec<(Stopwatch, Instant)>>,
    current_timer: Option<String>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            timers: HashMap::new(),
            current_timer: None,
        }
    }

    pub fn start(&mut self, name: &str) {
        if let Some(current) = &self.current_timer {
            panic!("Timer '{}' was never stopped.", current);
        }

        self.current_timer = Some(name.to_string());
        self.timers.entry(name.to_string()).or_default().push((Stopwatch::start_new(), Instant::now()));
    }

    pub fn stop(&mut self) {
        if let Some(current) = &self.current_timer {
            if let Some((current_timer, _)) = self.timers.get_mut(current).and_then(|timers| timers.last_mut()) {
                current_timer.stop();
                self.current_timer = None;
            }
        } else {
            panic!("No timer running.");
        }
    }

    pub fn cleanup(&mut self) {
        let three_seconds = Duration::new(WINDOW_SEC, 0);
        let now = Instant::now();

        for timers in self.timers.values_mut() {
            timers.retain(|(stopwatch, start_time)| {
                stopwatch.is_running() || now.duration_since(*start_time) < three_seconds
            });
        }
    }

    pub fn get_analysis(&self) -> RuntimeAnalysis{
        let mut names = Vec::new();
        let mut counts = Vec::new();
        let mut totals = Vec::new();
        let mut averages = Vec::new();

        for name in self.timers.keys() {
            let mut total = 0;
            let count = self.timers[name].len();

            for (stopwatch, _) in self.timers[name].iter() {
                total += stopwatch.elapsed().as_millis();
            }

            names.push(name.clone());
            counts.push(count);
            totals.push(total);
            averages.push(total as f32 / count as f32);            
        }
        
        RuntimeAnalysis {
            names,
            counts,
            totals,
            averages
        }
    }
}
