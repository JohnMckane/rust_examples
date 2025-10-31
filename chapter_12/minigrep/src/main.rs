use std::fs;
use std::env;
use std::error::Error;
use std::process;
use minigrep::search;
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
    file_path: String
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args")
        }
        Ok(Config {query: args[1].clone(), file_path: args[2].clone()})
    }
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}
