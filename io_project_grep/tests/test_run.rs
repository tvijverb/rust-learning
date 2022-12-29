
use io_project_grep::{Config, search, search_case_insensitive};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let poem = [String::from("poem.txt"), String::from("poem.txt"), String::from("poem.txt")];
        let query = "duct";
        let contents = "\n nom?";
        Config::new(&poem);
        assert!(true);
    }

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}