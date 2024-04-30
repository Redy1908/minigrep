use colored::{ColoredString, Colorize};
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[derive(Debug)]
pub struct Response {
    line_start: ColoredString,
    line_match: ColoredString,
    line_end: ColoredString,
}

impl Response {
    fn new(line: &str, query_len: usize, query_match_start: usize) -> Response {
        let query_match_end = query_match_start + query_len;

        let line_start = &line[0..query_match_start];
        let line_match = &line[query_match_start..query_match_end];
        let line_end = &line[query_match_end..];

        Response {
            line_start: line_start.white(),
            line_match: line_match.red(),
            line_end: line_end.white(),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let responses = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for response in responses {
        println!(
            "{}{}{}",
            response.line_start, response.line_match, response.line_end
        );
    }

    Ok(())
}

pub fn search(query: &str, contents: &str) -> Vec<Response> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if let Some(query_match_start) = line.find(query) {
            results.push(Response::new(line, query.len(), query_match_start))
        }
    }

    results
}

pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<Response> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if let Some(query_match_start) = line.to_lowercase().find(&query) {
            results.push(Response::new(line, query.len(), query_match_start));
        }
    }

    results
}
