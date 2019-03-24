use std::env;
use std::process;

use dice_roller;
use dice_roller::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Config: {:?}", config);

    if let Err(e) = dice_roller::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
