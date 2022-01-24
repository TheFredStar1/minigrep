use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    user super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents="\
        Rust:
        safe, fast, productive.
        Pick three.";
    }

    assert_eq!(vec!["safe", "fast", "productive"], search(query, contents));
}

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // TODO: Refactor to not use clone
        let query = args[1].clone();
        let filename = args[2].clone();

        let config = Config {
            query: query,
            filename: filename
        };

        Ok(config)

    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text: \n{}", contents);

    Ok(())
}