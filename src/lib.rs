extern crate rand;

use std::error::Error;
use rand::distributions::{Distribution, Uniform};

pub struct Config {
    pub rolls: u32,
    pub die: u32,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let roll = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a dice roll"),
        };

        let parts: Vec<&str> = roll.split("d").collect();
        if parts.len() != 2 {
            return Err("Dice roll must be of the format: <num_of_rolls>d<sides_of_die>");
        }

        let parse_err = "Numbers of rolls and die type must both be positive integers";
        let mut parts = parts.iter();
        let rolls = parts.next().unwrap().parse::<u32>();
        let die = parts.next().unwrap().parse::<u32>();
        if rolls.is_err() || die.is_err() {
            return Err(parse_err);
        }

        let rolls = rolls.unwrap();
        let die = die.unwrap();
        if rolls == 0 || die == 0 {
            return Err(parse_err);
        }

        Ok(Config { rolls, die })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let upper_bound = config.die + 1;
    let die = Uniform::from(1..upper_bound);

    let mut total = 0;
    for x in 0..config.rolls {
        let throw = die.sample(&mut rng);
        total += throw;
        println!("Throw {}: {}", x + 1, throw);
    }
    println!("Total: {}", total);

    Ok(())
}
