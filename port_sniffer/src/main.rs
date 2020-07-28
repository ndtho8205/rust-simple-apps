mod config;
mod sniffer;

use std::env::args;
use std::sync::mpsc;
use std::thread;

use crate::config::Config;

const MAX_PORT: u16 = 65535;

fn main() -> Result<(), &'static str> {
    let config = Config::parse(args().collect())?;

    let mut opened_ports = run(&config);
    opened_ports.sort();
    println!("Opened ports: {:?}", opened_ports);

    Ok(())
}

fn run(config: &Config) -> Vec<u16> {
    let (tx, rx) = mpsc::channel();
    let mut opened_ports = vec![];

    let ip_addr = config.ip;
    let threads = if config.threads == 0 {
        1
    } else {
        config.threads
    };

    for id in 1..=threads {
        let cloned_tx = tx.clone();

        thread::spawn(move || {
            let step = id as u16;
            let mut port = step;

            loop {
                if sniffer::scan(ip_addr, port) {
                    cloned_tx.send(port).unwrap();
                }

                port += step;

                if MAX_PORT - port <= step {
                    break;
                }
            }
        });
    }

    drop(tx);

    for port in rx {
        opened_ports.push(port);
    }

    opened_ports
}
