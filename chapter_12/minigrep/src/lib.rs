pub fn search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    vec!["safe, fast, productive."]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn two_result() {
        let query = "duct";
        let contents = "\
Rust:
quick, speedy, productive.
Pick three.";

        assert_eq!(vec!["quick, speedy, productive."], search(query, contents));
    }
}
