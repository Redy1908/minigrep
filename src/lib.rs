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

        let mut matches = matches.free.into_iter();

        let pattern = match matches.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a pattern string".to_string()),
        };

        let file_path = match matches.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path".to_string()),
        };

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let mathing_lines = if config.case_insensitive {
        search_case_insensitive(&config.pattern, &contents)
    } else {
        search_case_sensitive(&config.pattern, &contents)
    };

    mathing_lines.iter().for_each(|matching_line| {
        if config.print_line_index {
            print!("{}", format!("{}:", matching_line.line_number).green());
        }

        print_line(matching_line);
    });

    Ok(())
}

fn print_line(line: &MatchingLine) {
    line.parts.iter().enumerate().for_each(|(index, part)| {
        print!("{}", format!("{}", part.white()));

        if index != line.parts.len() - 1 {
            print!("{}", line.pattern.red());
        }
    });
    println!();
}

fn search_case_sensitive(pattern: &str, contents: &str) -> Vec<MatchingLine> {
    contents
        .lines()
        .filter(|line| line.contains(pattern))
        .enumerate()
        .map(|(index, line)| {
            let parts = line.split(&pattern).map(String::from).collect();
            MatchingLine {
                line_number: (index + 1) as u32,
                parts,
                pattern: pattern.to_string(),
            }
        })
        .collect()
}

fn search_case_insensitive(pattern: &str, contents: &str) -> Vec<MatchingLine> {
    let pattern_lowercase = pattern.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern_lowercase))
        .enumerate()
        .map(|(index, line)| {
            let parts = split_line(&line, &pattern_lowercase);
            let original_pattern = get_original_pattern(&line, &pattern_lowercase);
            MatchingLine {
                line_number: (index + 1) as u32,
                parts,
                pattern: original_pattern,
            }
        })
        .collect()
}

fn split_line(line: &str, pattern_lowercase: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut last_pos = 0;
    let mut pos = line.to_lowercase().find(pattern_lowercase);
    while let Some(p) = pos {
        parts.push(line[last_pos..p].to_string());
        last_pos = p + pattern_lowercase.len();
        pos = line.to_lowercase()[last_pos..]
            .find(pattern_lowercase)
            .map(|p| p + last_pos);
    }
    parts.push(line[last_pos..].to_string());
    parts
}

fn get_original_pattern(line: &str, pattern_lowercase: &str) -> String {
    let start = line.to_lowercase().find(pattern_lowercase).unwrap();
    line[start..start + pattern_lowercase.len()].to_string()
}
