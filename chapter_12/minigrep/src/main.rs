use std::fs;
use std::env;
use std::error::Error;
use std::process;
use minigrep::{search, search_case_insensitive};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap();
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args")
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        return Ok(Config {query: args[1].clone(), file_path: args[2].clone(), ignore_case: ignore_case})
    }
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}
