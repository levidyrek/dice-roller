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
    let (throws, total) = get_throws(config.die, config.rolls);
    for (index, throw) in throws.iter().enumerate() {
        println!("Throw {}: {}", index + 1, throw);
    }
    println!("Total: {}", total);

    Ok(())
}

fn get_throws(die: u32, rolls: u32) -> (Vec<u32>, u32) {
    let mut rng = rand::thread_rng();
    let upper_bound = die + 1;
    let die = Uniform::from(1..upper_bound);

    let mut throws = Vec::new();
    let mut total = 0;
    for _x in 0..rolls {
        let throw = die.sample(&mut rng);
        total += throw;
        throws.push(throw);
    }

    (throws, total)
}

mod tests {
    use super::*;

    #[test]
    fn test_get_throws() {
        let (throws, total) = get_throws(10, 10);
        assert_eq!(throws.len(), 10);
        let mut tot = 0;
        for throw in throws {
            assert!(throw < 11);
            assert!(throw > 0);
            tot += throw;
        }
        assert_eq!(total, tot);
    }
}
