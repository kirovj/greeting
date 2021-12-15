use std::env::Args;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    /**
     * use Result instead of Object
     */
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("query is none"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("filename is none"),
        };
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = search(&config.query, &contents);
    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
lang.
Duct tape";

        assert_eq!(vec!["safe, fast, productive"], search(&query, &contents));
    }
}
