use select::document::Document;
use select::predicate::{Child, Name};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn get_blockquote_p(html: &str) -> Vec<String> {
    Document::from(html)
        .find(Child(Name("blockquote"), Name("p")))
        .map(|node| node.text().replace("”", "").replace("“", ""))
        .collect::<Vec<String>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("quote-db.txt")?;

    let sites = vec![
        "https://animemotivation.com/vegeta-quotes/",
        "https://animemotivation.com/dragon-ball-z-quotes/",
    ];
    for site in sites {
        let body = reqwest::blocking::get(site)?.text()?;

        let result = get_blockquote_p(&body);
        for r in result {
            if let Err(e) = writeln!(file, "{}", r) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
    Ok(())
}
