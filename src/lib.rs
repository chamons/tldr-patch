use std::collections::HashSet;
use std::io::Read;

use error_chain::error_chain;
use regex::Regex;
use std::result::Result as StdResult;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Regex(regex::Error);
    }
}

struct FileFilter {
    filters: Option<Vec<Regex>>,
}

impl FileFilter {
    pub fn init(filter: &Option<String>) -> Result<FileFilter> {
        let filters: Option<Vec<Regex>> = if let Some(filter) = filter {
            // error_chain's Result gave me grief here
            let filters: StdResult<_, _> = std::fs::read_to_string(filter)?.lines().map(|l| Regex::new(l)).collect();
            Some(filters?)
        } else {
            None
        };
        Ok(FileFilter { filters })
    }

    pub fn should_be_filtered(&self, file_name: &str) -> bool {
        if file_name.contains("/dev/null") {
            true
        } else if let Some(filters) = &self.filters {
            filters.iter().any(|f| f.is_match(file_name))
        } else {
            file_name.contains("Generated") || file_name.contains("SessionRecords")
        }
    }
}

fn patch_line_to_file_name(line: &str) -> &str {
    let file = &line[4..];
    if file.starts_with("a/") || file.starts_with("b/") {
        &file[2..]
    } else {
        &file
    }
}

fn calculate_modified_files<'a>(body: &'a str, filter: &Option<String>) -> Result<HashSet<&'a str>> {
    let filter = FileFilter::init(filter)?;
    let mut files = HashSet::new();
    for line in body.lines() {
        if line.starts_with("+++ ") || line.starts_with("--- ") {
            let file = patch_line_to_file_name(&line);
            if !filter.should_be_filtered(file) {
                files.insert(file);
            }
        }
    }
    Ok(files)
}

fn get_modified_files(body: &str, filter: &Option<String>) -> Result<Vec<String>> {
    let mut files: Vec<String> = calculate_modified_files(&body, filter)?.iter().map(|s| s.to_string()).collect();
    files.sort();
    Ok(files)
}

pub fn print_modified_filed(body: &str, filter: &Option<String>) -> Result<()> {
    for file in &get_modified_files(body, filter)? {
        println!("{}", file);
    }
    Ok(())
}

pub fn print_diff(body: &str, filter: &Option<String>) -> Result<()> {
    let files = calculate_modified_files(body, filter)?;
    let mut should_print = false;
    for line in body.lines() {
        if line.starts_with("+++ ") || line.starts_with("--- ") {
            let file = patch_line_to_file_name(&line);
            should_print = files.contains(file);
        }
        if should_print {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn fetch_pr(url: &str) -> Result<String> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    Ok(body)
}
