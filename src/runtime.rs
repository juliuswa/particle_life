use stopwatch::Stopwatch;
use std::collections::HashMap;

pub struct Runtime {
    timers: HashMap<String, Vec<Stopwatch>>,
    current_timer: Option<Stopwatch>
}

impl Runtime {
    pub fn new() -> Self{
        Self {
            timers: HashMap::new(),
            current_timer: None,
        }
    }

    pub fn start(&self, name: &str) {

    }
}