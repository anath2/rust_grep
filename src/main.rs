use std::error::Error;
use std::{env, fs, process};


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problems parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);

    }
}


struct Config {
    query: String,
    filename: String
}


impl Config {

    fn new(args: &[String]) -> Result <Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();


        Ok(Config {query, filename})
    }

}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("File contents {}", contents);
    Ok(())
}
