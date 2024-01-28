use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to output file.
    #[arg(short, long, default_value = "highlights.org")]
    pub output_file: String,

    /// Export type. Options are 'org', 'md', and 'txt'.
    #[arg(short, long, default_value = "org")]
    pub export_type: String,

    /// Path to Kindle clippings file.
    #[arg(short, long)]
    pub clippings_path: Option<String>,
}

pub fn default_kindle_clippings_path() -> Option<String> {
    let home = match env::var("HOME") {
        Ok(h) => h,
        Err(_) => return None,
    };

    let config_file = format!("{}/.config/kindle2doc/kindle_clippings_path", home);

    match fs::read_to_string(&config_file) {
        Ok(path) => Some(path.trim().to_string()),
        Err(_e) => None,
    }
}
pub fn clean_title(title: &str) -> &str {
    if title.starts_with("\u{feff}") {
        &title[3..]
    } else {
        title
    }
}

pub fn write_file(highlights: HashMap<String, Vec<String>>, output_file: &str, export_type: &str) {
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
