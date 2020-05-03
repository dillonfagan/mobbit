use std::time::Duration;
use stopwatch::Stopwatch;

pub struct Turn {
    minutes: u64,
    duration: Duration,
    stopwatch: Stopwatch,
}

impl Turn {
    pub fn new(minutes: u64) -> Turn {
        Turn {
            minutes: minutes,
            duration: Duration::from_secs(minutes * 60),
            stopwatch: Stopwatch::new(),
        }
    }

    pub fn start(&mut self) {
        self.stopwatch.start();
        println!("Turn started. {} minutes remaining.", self.minutes);
    }
}