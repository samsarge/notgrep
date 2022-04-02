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
    // take ownership of args and mutating it because we'll be iterating over it
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // first is the program name so skip that

        let text_to_search = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };
        // will return the Err variant of Result if the env var isnt set
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { text_to_search, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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