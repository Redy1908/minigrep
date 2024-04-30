extern crate getopts;
use colored::Colorize;
use getopts::Options;
use std::error::Error;
use std::fs;

pub struct Config {
    pub pattern: String,
    pub file_path: String,
    pub case_insensitive: bool,
    pub print_line_index: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        let mut opts = Options::new();

        opts.optflag("i", "ignore case", "Case insensitive pattern matching.");

        opts.optflag("n", "line number", "Show the line number.");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(e) => {
                let error = format!("Not valid params {}", e);
                return Err(error);
            }
        };

        let case_insensitive = matches.opt_present("i").then_some(true).unwrap_or(false);
        let print_line_index = matches.opt_present("n").then_some(true).unwrap_or(false);

        if matches.free.len() != 2 {
            return Err("Not enough arguments".to_string());
        }

        let pattern = matches.free[0].clone();
        let file_path = matches.free[1].clone();

        Ok(Config {
            pattern,
            file_path,
            case_insensitive,
            print_line_index,
        })
    }
}

#[derive(Default)]
pub struct MatchingLine {
    pub parts: Vec<String>,
    pub pattern: String,
    pub line_number: u32,
}

impl MatchingLine {
    fn new(pattern: &str) -> MatchingLine {
        MatchingLine {
            pattern: pattern.to_string(),
            ..Default::default()
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let lines = if config.case_insensitive {
        search_case_insensitive(&config.pattern, &contents)
    } else {
        search_case_sensitive(&config.pattern, &contents)
    };

    for i in 0..lines.len() {
        if config.print_line_index {
            print!("{}", format!("{}:", lines[i].line_number).green());
        }

        for j in 0..lines[i].parts.len() {
            print!("{}", lines[i].parts[j].white());

            if j != lines[i].parts.len() - 1 {
                print!("{}", lines[i].pattern.red());
            }
        }
        println!();
    }

    Ok(())
}

pub fn search_case_sensitive(pattern: &str, contents: &str) -> Vec<MatchingLine> {
    let mut results = Vec::new();

    for (index, file_line) in contents.lines().enumerate() {
        let mut matching_line = MatchingLine::new(&pattern);

        let parts: Vec<_> = file_line.split(&pattern).collect();

        if parts.len() > 1 {
            for part in parts {
                matching_line.line_number = (index + 1) as u32;
                matching_line.parts.push(part.to_string());
            }

            results.push(matching_line);
        }
    }

    results
}

pub fn search_case_insensitive(pattern: &str, contents: &str) -> Vec<MatchingLine> {
    let pattern = pattern.to_lowercase();
    let mut results = Vec::new();

    for (index, file_line) in contents.to_lowercase().lines().enumerate() {
        let mut matching_line = MatchingLine::new(&pattern);

        let parts: Vec<_> = file_line.split(&pattern).collect();

        if parts.len() > 1 {
            for part in parts {
                matching_line.line_number = (index + 1) as u32;
                matching_line.parts.push(part.to_string());
            }

            results.push(matching_line);
        }
    }

    results
}
