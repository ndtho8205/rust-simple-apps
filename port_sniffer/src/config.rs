use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug)]
pub struct Config {
    pub print_usage: bool,
    pub threads: usize,
    pub ip: IpAddr,
}

impl Config {
    pub fn parse(args: Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let mut print_usage = false;
        let mut threads = 0;
        let mut ip: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-h" => {
                    print_usage = true;
                    i += 1;
                }
                "-j" => {
                    match args.get(i + 1) {
                        Some(n) if n.parse::<usize>().is_ok() => {
                            threads = n.parse::<usize>().unwrap();
                            i += 2;
                        }
                        _ => return Err("number of threads is not provided"),
                    };
                }
                flag if flag.starts_with("-") => return Err("unrecognized argument"),
                _ => {
                    match args[i].parse::<IpAddr>() {
                        Ok(addr) => ip = addr,
                        Err(_) => return Err("not a valid IpAddr"),
                    };
                    i += 1;
                }
            }
        }

        if !print_usage && ip.is_unspecified() {
            return Err("IpAddr is not provided");
        }

        Ok(Config {
            print_usage,
            threads,
            ip: ip,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_error_when_no_argument_provided() {
        let args = vec![];
        let config = Config::parse(args);
        assert_eq!(config.is_err(), true);
        assert_eq!(config.err(), Some("not enough arguments"));
    }

    #[test]
    fn return_error_on_unrecognized_argument() {
        let args = vec![String::from("path"), String::from("-v")];
        let config = Config::parse(args);

        assert_eq!(config.is_err(), true);
        assert_eq!(config.err(), Some("unrecognized argument"));
    }

    #[test]
    fn return_error_when_no_ip_provided() {
        let args = vec![String::from("path"), String::from("-j"), String::from("10")];
        let config = Config::parse(args);
        assert_eq!(config.is_err(), true);
        assert_eq!(config.err(), Some("IpAddr is not provided"))
    }

    #[test]
    fn parse_help() {
        let args = vec![String::from("path"), String::from("-h")];
        let config = Config::parse(args).unwrap();
        assert_eq!(config.print_usage, true);
    }

    #[test]
    fn parse_ip() {
        let args = vec![String::from("path"), String::from("127.0.0.1")];
        let config = Config::parse(args).unwrap();

        assert_eq!(config.ip, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(config.threads, 0);
        assert_eq!(config.print_usage, false);
    }

    #[test]
    fn parse_threads_and_ip() {
        let args = vec![
            String::from("path"),
            String::from("-j"),
            String::from("10"),
            String::from("127.0.0.1"),
        ];
        let config = Config::parse(args).unwrap();

        assert_eq!(config.ip, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(config.threads, 10);
        assert_eq!(config.print_usage, false);
    }

    #[test]
    fn return_error_number_of_threads_is_not_provided() {
        let args = vec![
            String::from("path"),
            String::from("-j"),
            String::from("127.0.0.1"),
        ];
        let config = Config::parse(args);

        assert_eq!(config.is_err(), true);
        assert_eq!(config.err(), Some("number of threads is not provided"));
    }

    #[test]
    fn parse_all_flags() {
        let args = vec![
            String::from("path"),
            String::from("127.0.0.1"),
            String::from("-j"),
            String::from("10"),
            String::from("-h"),
        ];
        let config = Config::parse(args).unwrap();

        assert_eq!(config.print_usage, true);
        assert_eq!(config.threads, 10);
        assert_eq!(config.ip, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }
}
