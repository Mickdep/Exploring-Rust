use std::env;
use std::process;

use chapter_12::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); //Get command line args
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = chapter_12::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
