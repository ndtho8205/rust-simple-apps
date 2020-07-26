pub fn grep<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_one_result() {
        let query = "language";
        let contents =
            "Rust\nA language empowering everyone\nto build reliable and efficient software.";
        assert_eq!(
            vec!["A language empowering everyone"],
            grep(query, contents)
        );
    }
}
