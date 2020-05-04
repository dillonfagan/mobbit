use std::time::Duration;
use std::thread;

pub struct Turn {
    minutes: u64,
    duration: Duration,
}

impl Turn {
    pub fn new(minutes: u64) -> Turn {
        Turn {
            minutes: minutes,
            duration: Duration::from_secs(minutes * 60),
        }
    }

    pub fn start(&self) {
        println!("Your turn started. {} minutes remaining.", self.minutes);
        thread::sleep(self.duration);
        println!("Your turn is over. Time to commit!");
    }
}