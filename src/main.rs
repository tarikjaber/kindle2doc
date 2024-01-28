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

    let mut highlights: HashMap<String, Vec<String>> = HashMap::new();

    for part in parts {
        let lines: Vec<&str> = part.lines().filter(|line| !line.is_empty()).collect();

        if lines.len() > 1 {
            let title = utils::clean_title(lines[0].split(" (").collect::<Vec<&str>>()[0]);
            let highlight = lines.last().unwrap();
            highlights.entry(title.to_string()).or_insert(Vec::new()).push(highlight.to_string());
        }
    }

    fs::File::create("highlights.txt").expect("Unable to create file");

    let mut titles: Vec<String> = highlights.keys().cloned().collect();
    titles.sort();

    utils::write_file(highlights, &args.output_file, &args.export_type);
}
