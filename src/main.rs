use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to output file.
    #[arg(short, long, default_value = "highlights.org")]
    output_file: String,

    /// Export type. Options are 'org', 'md', and 'txt'.
    #[arg(short, long, default_value = "org")]
    export_type: String,

    /// Path to Kindle clippings file.
    #[arg(short, long, default_value_t = default_kindle_clippings_path())]
    clippings_path: String,
}

fn default_kindle_clippings_path() -> String {
    let home = match env::var("HOME") {
        Ok(h) => h,
        Err(_) => panic!("Unable to read $HOME"),
    };
    let config_file = format!("{}/.config/kindle2doc/kindle_clippings_path", home);

    match fs::read_to_string(&config_file) {
        Ok(path) => path.trim().to_string(),
        Err(_e) => panic!("Unable to read kindle_clippings_path from {}", config_file),
    }
}

fn clean_title(title: &str) -> &str {
    if title.starts_with("\u{feff}") {
        &title[3..]
    } else {
        title
    }
}

fn write_file(highlights: HashMap<String, Vec<String>>, output_file: &str, export_type: &str) {
    let mut titles: Vec<String> = highlights.keys().cloned().collect();
    titles.sort();

    let mut file = fs::File::create(output_file).expect("Unable to create file");
    let title_prefix = if export_type == "org" { "* " } else { "# " };
    
    for title in titles {
        file.write_all(format!("{}{}\n", title_prefix, title).as_bytes()).expect("Unable to write data");

        for highlight in highlights.get(&title).unwrap() {
            file.write_all(format!("- {}\n", highlight).as_bytes()).expect("Unable to write data");
        }
    }
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.clippings_path).expect("Something went wrong reading the file");
    let parts = contents.split("==========");

    let mut highlights: HashMap<String, Vec<String>> = HashMap::new();

    for part in parts {
        let lines: Vec<&str> = part.lines().filter(|line| !line.is_empty()).collect();

        if lines.len() > 1 {
            let title = clean_title(lines[0].split(" (").collect::<Vec<&str>>()[0]);
            let highlight = lines.last().unwrap();
            highlights.entry(title.to_string()).or_insert(Vec::new()).push(highlight.to_string());
        }
    }

    fs::File::create("highlights.txt").expect("Unable to create file");

    let mut titles: Vec<String> = highlights.keys().cloned().collect();
    titles.sort();

    write_file(highlights, &args.output_file, &args.export_type);
}
