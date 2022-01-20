use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    
    // Exit with error
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}

// Struct does not implement Copy
// so values inside will "Move"
struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
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