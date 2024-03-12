mod utils;

use std::collections::HashMap;
use clap::Parser;
use std::fs;

fn main() {
    let args = utils::Args::parse();
    let contents = match args.clippings_path {
        Some(path) => fs::read_to_string(path).expect("Something went wrong reading the clippings path file"),
        None => {
            let path = utils::default_kindle_clippings_path(); 
            let kindle_path = match path {
                Some(p) => p,
                None => panic!("No clippings path provided and no default path found"), 
            };
            fs::read_to_string(kindle_path).expect("Something went wrong reading the clippings path file")
        }
    };

    let parts = contents.split("==========");

    let mut highlights: HashMap<&str, Vec<&str>> = HashMap::new();

    for part in parts {
        let lines: Vec<&str> = part.lines().filter(|line| !line.is_empty()).collect();

        if lines.len() > 1 {
            let title = utils::clean_title(lines[0].split(" (").next().expect("There should be a title"));
            let highlight = lines.last().unwrap();
            highlights.entry(title).or_default().push(highlight);
        }
    }

    let mut titles: Vec<&str> = highlights.keys().cloned().collect();
    titles.sort();

    utils::write_files(highlights, &args.export_type);
}
