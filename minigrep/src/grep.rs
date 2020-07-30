use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn grep<'a>(pattern: &str, path: &PathBuf) -> Result<Vec<String>, String> {
    let reader = match File::open(path) {
        Ok(f) => BufReader::new(f),
        Err(e) => return Err(e.to_string()),
    };

    let mut results: Vec<String> = vec![];

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(pattern) {
                results.push(line);
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_error_no_such_file() {
        let result = grep("", &PathBuf::from("tests/fixtures/not_found.txt"));

        assert_eq!(result.is_err(), true);
        assert_eq!(result.err().unwrap().contains("No such file"), true);
    }

    #[test]
    fn grep_found_sucessful() {
        let result = grep("Rust", &PathBuf::from("tests/fixtures/why_rust.txt")).unwrap();

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn grep_not_found_sucessful() {
        let result = grep("Hello", &PathBuf::from("tests/fixtures/why_rust.txt")).unwrap();

        assert_eq!(result.len(), 0);
    }
}
