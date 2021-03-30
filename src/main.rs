use std::io::Read;
use std::process;
use std::{collections::HashSet, env};

use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn fetch(url: &str) -> Result<String> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    Ok(body)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Problem parsing arguments. Expected URL");
        process::exit(1);
    }
    let body = fetch(args.get(1).unwrap())?;

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
