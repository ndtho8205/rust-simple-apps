mod config;
mod grep;

use std::env;
use std::fs;

use crate::config::Config;
use crate::grep::grep;

fn main() -> Result<(), String> {
    let config = Config::parse(&env::args().collect::<Vec<String>>())?;

    run(&config)?;

    Ok(())
}

fn run(config: &Config) -> Result<(), String> {
    let contents = match fs::read_to_string(&config.filename()) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };

    let results = grep(config.query(), &contents);

    if results.len() == 0 {
        println!("Not found.");
        return Ok(());
    }

    for line in grep(config.query(), &contents) {
        println!("{}", line);
    }

    Ok(())
}
