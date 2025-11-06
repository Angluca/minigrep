use std::{env,process};
use minigrep::{Config};
fn main() {
    //let mut args: Vec<String> = env::args().collect();
    //let config = Config::build(args.iter()).unwrap_or_else(|err| {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
