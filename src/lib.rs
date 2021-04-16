use std::collections::HashSet;
use std::io::Read;

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
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

fn calculate_modified_files(body: &str) -> HashSet<&str> {
    let mut files = HashSet::new();
    for line in body.lines() {
        if line.starts_with("+++ ") || line.starts_with("--- ") {
            let file = patch_line_to_file_name(&line);
            if !file.contains("Generated") && !file.contains("SessionRecords") && !file.contains("/dev/null") {
                files.insert(file);
            }
        }
    }
    files
}

fn get_modified_files(body: &str) -> Vec<String> {
    let mut files: Vec<String> = calculate_modified_files(&body)
        .iter()
        .map(|s| s.to_string())
        .collect();
    files.sort();
    files
}

pub fn print_modified_filed(body: &str) {
    for file in &get_modified_files(body) {
        println!("{}", file);
    }
}

pub fn print_diff(body: &str) {
    let files = calculate_modified_files(body);
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
}

pub fn fetch_pr(url: &str) -> Result<String> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    Ok(body)
}
