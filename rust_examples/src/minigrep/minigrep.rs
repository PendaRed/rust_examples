use std::error::Error;
use std::fs;
use std::env;

/* eg of using this
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let c = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parseing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", c.query);
    println!("In file {}", c.filename);

    if let Err(e) = minigrep::run(c) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
 */

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough argumants");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_insensitive })
    }
}

pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(c.filename)?; // sugar for match err or ok, and err passes err up with return
println!("{}", c.case_insensitive);
    let results = if c.case_insensitive {
        search(&c.query, &contents)
    } else {
        search_case_insensitive(&c.query, &contents)
    };

    for line in  results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*; // glob

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // "\ means dont put newline before the string literal
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        // "\ means dont put newline before the string literal
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
