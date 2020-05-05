use std::time::Duration;
use std::thread;
use console::style;
use indicatif::{ ProgressBar, ProgressStyle };

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
        let seconds = self.duration.as_secs();

        let bar = ProgressBar::new(seconds);
        bar.set_style(ProgressStyle::default_bar()
            .template("[{eta_precise}] {bar:50.cyan/blue} {msg}")
            .progress_chars("== "));

        for _ in 0..seconds {
            thread::sleep(Duration::from_secs(1));
            bar.inc(1);
        }

        bar.finish();
        bar.set_message("Time to commit!");
    }
}