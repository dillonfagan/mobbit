pub struct Turn {
    minutes: u8,
}

impl Turn {
    pub fn new(minutes: u8) -> Turn {
        Turn {
            minutes
        }
    }

    pub fn start(&self) {
        println!("Turn started. {} minutes remaining.", self.minutes);
    }
}