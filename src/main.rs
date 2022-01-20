use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();  

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arugments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text: \n{}", contents);

    Ok(())
}