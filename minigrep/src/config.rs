pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn parse(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].to_owned();
        let filename = args[2].to_owned();

        Ok(Self { query, filename })
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_error_not_enough_arguments() {
        let config = Config::parse(&vec![]);
        assert_eq!(config.is_err(), true);
        assert_eq!(config.err(), Some("not enough arguments"));
    }

    #[test]
    fn parse_ok() {
        let config = Config::parse(&vec![
            "path".to_owned(),
            "query".to_owned(),
            "filename".to_owned(),
        ]);
        assert_eq!(config.is_ok(), true);

        let config = config.unwrap();
        assert_eq!(config.query(), "query");
        assert_eq!(config.filename(), "filename");
    }
}
