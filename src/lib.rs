use std::{env,fs,error::Error};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    //pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //dbg!(&args);
        //let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        //if args.len() < 3 {
            //return Err("not enough arguments");
        //}
        //else if args.len() > 3 {
            //ignore_case =
                //if args[3].parse::<u8>().unwrap_or(0) != 0 {true} else {false};
        //}
        //let query = args[2].to_string();
        //let file_path = args[1].to_string();
        //Ok(Config { query, file_path, ignore_case })
    //}
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = match args.next() {
            Some(arg) => if arg.parse::<u8>().unwrap_or(0) != 0 { true } else { false },
            None => env::var("IGNORE_CASE").is_ok(),
        };
        Ok(Config { query, file_path, ignore_case })
    }
}
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    let contents = fs::read_to_string(config.file_path.as_str())?;
    //println!("With text:\n{contents}");
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
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
//pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let mut results = Vec::new();
    //for line in contents.lines() {
        //if line.contains(query) {
            //results.push(line);
        //}
    //}
    //results
//}
//pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //let query = query.to_lowercase();
    //let mut results = Vec::new();
    //for line in contents.lines() {
        //if line.to_lowercase().contains(&query) {
            //results.push(line);
        //}
    //}
    //results
//}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
