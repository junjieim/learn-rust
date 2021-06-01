use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).expect("aaa ");

    let content = fs::read_to_string(config.filename)
        .expect("Something wrong reading the file.");

    println!("{}", content);
}

#[allow(dead_code)]
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}
