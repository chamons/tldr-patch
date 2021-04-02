use std::io::Read;
use std::{collections::HashSet};

use error_chain::error_chain;
use clap::Clap;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Clap)]
#[clap(version = "0.1", author = "Chris Hamons <chris.hamons@gmail.com>")]
struct Options {
    url: String,
}

fn fetch(url: &str) -> Result<String> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    Ok(body)
}

fn main() -> Result<()> {
    let options: Options = Options::parse();

    let url = options.url;
    let url = if url.ends_with(".diff") {
        url.to_string()
    } else {
        format!("{}.diff", url)
    };
    let body = fetch(&url)?;

    let mut files = HashSet::new();
    for line in body.lines() {
        if line.starts_with("+++ ") || line.starts_with("--- ") {
            let file = &line[4..];
            let file = if file.starts_with("a/") || file.starts_with("b/") {
                &file[2..]
            } else {
                &file
            };
            if !file.contains("Generated") && !file.contains("/dev/null") {
                files.insert(file);
            }
        }
    }

    let mut files: Vec<String> = files.iter().map(|s| s.to_string()).collect();
    files.sort();
    for file in &files {
        println!("{}", file);
    }

    Ok(())
}
