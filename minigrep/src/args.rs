use std::path::PathBuf;

pub struct Cli {
    pub help: bool,
    pub pattern: String,
    pub path: PathBuf,
}

impl Cli {
    pub fn parse(args: &Vec<String>) -> Result<Self, &'static str> {
        let help = !args
            .iter()
            .find(|&arg| arg == "-h" || arg == "--help")
            .is_none();
        let mut pattern = "".to_string();
        let mut path = PathBuf::new();

        if !help {
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            pattern = args[1].to_owned();
            path = PathBuf::from(args[2].to_owned());
        }

        Ok(Self {
            help,
            pattern,
            path,
        })
    }

    pub fn print_usage(&self) {
        println!(
            "\
usage: minigrep [OPTION] <pattern> <path>

Options:
  -h, --help    Print help and exit"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_error_not_enough_arguments() {
        let cli = Cli::parse(&vec![String::from("path")]);
        assert_eq!(cli.is_err(), true);
        assert_eq!(cli.err(), Some("not enough arguments"));
    }

    #[test]
    fn parse_pattern_path_sucessful() {
        let cli = Cli::parse(&vec![
            String::from("path"),
            String::from("query"),
            String::from("test.txt"),
        ])
        .unwrap();

        assert_eq!(cli.help, false);
        assert_eq!(cli.pattern, "query");
        assert_eq!(cli.path, PathBuf::from("test.txt"));
    }

    #[test]
    fn parse_help_sucessful() {
        let cli = Cli::parse(&vec![String::from("path"), String::from("-h")]).unwrap();

        assert_eq!(cli.help, true);
        assert_eq!(cli.pattern, "");
        assert_eq!(cli.path, PathBuf::new());
    }
}
