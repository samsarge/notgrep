use std::env;
use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?; // the ? will return the Err immediately

    // search returns a vec of str slices
    let results = if config.case_sensitive {
        search(&config.text_to_search, &contents)
    } else {
        search_case_insensitive(&config.text_to_search, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    text_to_search: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let text_to_search: String = args[1].clone();
        let filename: String = args[2].clone();
        // will return the Err variant of Result if the env var isnt set
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { text_to_search, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }

    matches
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // shadow the query variable, to_lowercase() returns a new String; not a &str.
    let query = query.to_lowercase();
    let mut matches = Vec::new();

    for line in contents.lines() {
        // contains needs &str so pass a borrow of query
        if line.to_lowercase().contains(&query) {
            matches.push(line);
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "for";
        let contents = "\
This
is a test string
for the tests";

        assert_eq!(vec!["for the tests"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {

        let query = "tEsT";
        let contents = "\
This
is a TEST string
for the tests";

        assert_eq!(
            vec!["is a TEST string", "for the tests"],
            search_case_insensitive(query, contents)
        );
    }
}