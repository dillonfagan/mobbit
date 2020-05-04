use std::time::Duration;
use std::thread;
use console::style;

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
        println!("{} {} minutes remaining...", style("Turn started").green(), self.minutes);
        thread::sleep(self.duration);
        println!("{} Time to commit!", style("Turn finished").green());
    }
}