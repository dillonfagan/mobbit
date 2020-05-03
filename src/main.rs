use clap::{App, Arg, SubCommand};

fn start(minutes: u8) {
    println!("Mobbit started! Turns will be {} minutes long.", minutes);
}

fn main() {
    let matches = App::new("mobbit")
        .version("0.1.0")
        .about("The cool way to mob.")
        .author("Dillon Fagan")
        .subcommand(
            SubCommand::with_name("start")
                .about("Tells mobbit to start the driver's turn")
                .arg(
                    Arg::with_name("minutes")
                        .help("How long the driver's turn should be (default is 10)")
                        .index(1)
                        .default_value("10")
                        .required(false),
                ),
        )
        .get_matches();
    
    if let Some(matches) = matches.subcommand_matches("start") {
        let minutes: u8 = matches.value_of("minutes").unwrap().parse().unwrap();
        start(minutes);
    }

    println!("Mobbit has stopped.");
}