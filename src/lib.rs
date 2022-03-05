use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?; // the ? will return the Err immediately

    // search returns a vec of str slices
    for line in search(&config.text_to_search, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    text_to_search: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        Ok(Config { text_to_search: args[1].clone(), filename: args[2].clone() })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "for";
        let contents = "\
This
is a test string
for the tests";

        assert_eq!(vec!["for the tests"], search(query, contents));
    }
}