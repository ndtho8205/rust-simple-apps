use std::env;

mod args;
mod grep;

use crate::args::Cli;
use crate::grep::grep;

fn main() -> Result<(), String> {
    let cli = Cli::parse(&env::args().collect::<Vec<String>>())?;

    if cli.help {
        cli.print_usage();
        return Ok(());
    }

    run(&cli)?;

    Ok(())
}

fn run(cli: &Cli) -> Result<(), String> {
    let results = grep(&cli.pattern, &cli.path)?;

    if results.len() == 0 {
        println!("Not found!");
        return Ok(());
    }

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
