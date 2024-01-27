use std::{env,process};
use CLIgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
      eprintln!("Error parsing arguments --> {}",err);  //eprintln prints to the standard error stream
      process::exit(1);
    });
    println!("Searching for {}",config.query);
    println!("In file {}",config.file);
    if let Err(e) = CLIgrep::run(config) {
        eprintln!("Application Error {}",e);
        process::exit(1);
    }
}