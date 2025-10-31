use std::fs;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap();
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    run(config);
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
fn run(config: Config){
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
