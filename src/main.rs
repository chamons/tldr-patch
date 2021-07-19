use clap::Clap;

use tldr_patch::*;

#[derive(Clap)]
#[clap(version = "0.11", author = "Chris Hamons <chris.hamons@gmail.com>")]
struct Options {
    /// PR url to parse e.g. https://github.com/A/B/pull/1
    url: String,
    /// Instead of showing filed edited, show actual diffs
    #[clap(short, long)]
    patch: bool,
    /// File with one regular expression per line of files to ignore
    #[clap(short, long)]
    filter: Option<String>,
}

fn main() -> Result<()> {
    let options: Options = Options::parse();

    let url = options.url;
    let url = if url.ends_with(".diff") { url.to_string() } else { format!("{}.diff", url) };
    let body = fetch_pr(&url)?;

    if options.patch {
        print_diff(&body, &options.filter)?;
    } else {
        print_modified_filed(&body, &options.filter)?;
    }

    Ok(())
}
