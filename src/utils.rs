use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::env;

/// Program to convert Kindle clippings to a text, markdown, or org file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Export type. Options are 'org', 'md', and 'txt'.
    #[arg(short, long, default_value = "org")]
    pub export_type: String,

    /// Sets the output directory of the highlight files.
    #[arg(short, long, default_value = ".")]
    pub directory: String,

    /// Path to Kindle clippings file. If not provided will look for a default path in ~/.config/kindle2doc/kindle_clippings_path.
    #[arg(short, long)]
    pub clippings_path: Option<String>,

    /// Outputs all highlights into one file.
    #[arg(short, long)]
    pub one_file: bool,
}

pub fn default_kindle_clippings_path() -> Option<String> {
    let home = match env::var("HOME") {
        Ok(h) => h,
        Err(_) => return None,
    };

    let config_file = format!("{}/.config/kindle2doc/kindle_clippings_path", home);

    match fs::read_to_string(config_file) {
        Ok(path) => Some(path.trim().to_string()),
        Err(_e) => None,
    }
}

pub fn clean_title(title: &str) -> &str {
    if let Some(stripped) = title.strip_prefix('\u{feff}') {
        stripped
    } else {
        title
    }
}

pub fn write_file(highlights: HashMap<&str, Vec<&str>>, directory: &str, export_type: &str) {
    fs::create_dir_all(directory).expect("Unable to create highlights directory");

    let mut titles: Vec<&str> = highlights.keys().cloned().collect();
    titles.sort();

    let output_file = format!("{}/{}{}", directory, "highlights.", export_type);
    let mut file = fs::File::create(output_file).expect("Unable to create output file");
    let title_prefix = if export_type == "org" { "* " } else { "# " };
    
    for title in titles {
        file.write_all(format!("{}{}\n", title_prefix, title).as_bytes()).expect("Unable to write title");

        for highlight in highlights.get(title).unwrap() {
            file.write_all(format!("- {}\n", highlight).as_bytes()).expect("Unable to write highlights");
        }
    }
}

pub fn write_files(highlights: HashMap<&str, Vec<&str>>, directory: &str, export_type: &str) {
    fs::create_dir_all(directory).expect("Unable to create highlights directory");

    let mut titles: Vec<&str> = highlights.keys().cloned().collect();
    titles.sort();

    for title in titles {
        let file_name = title.split(':').next().unwrap_or("no_title");
        let output_file = format!("{}/{}.{}", directory, file_name, export_type);
        let mut file = fs::File::create(output_file).expect("Unable to create output file");

        for highlight in highlights.get(title).unwrap() {
            file.write_all(format!("- {}\n", highlight).as_bytes()).expect("Unable to write highlights");
        }
    }
}
