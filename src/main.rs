use minigrep::{search, search_case_sensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

// stdout (Standard Output)
// stderr (Standard Error)

// Program Task: Searching file and find the line in file that contain string user wants
fn main() {
    let args: Vec<String> = env::args().collect(); // the argument of the program // command line parsing logic
    // trying to run command cargo run -- needle haystack and you will get vector of  ["target/debug/minigrep", "needle", "haystack",]
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // prints any error to standard error instead standard output
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // make an associated function
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok(); // is_ok method will check wheter the IGNORE_CASE environment variable is set or not

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?; // automatically return from function if getting error and bring the result value
    let results = if config.ignore_case {
        search_case_sensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}
