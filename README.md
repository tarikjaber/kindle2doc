## kindle2doc
Converts kindle highlights into org, markdown, or text files.

## Usage
```
Program to convert Kindle clippings to a text, markdown, or org file

Usage: kindle2org [OPTIONS]

Options:
  -e, --export-type <EXPORT_TYPE>
          Export type. Options are 'org', 'md', and 'txt' [default: org]
  -d, --directory <DIRECTORY>
          Sets the output directory of the highlight files [default: .]
  -c, --clippings-path <CLIPPINGS_PATH>
          Path to Kindle clippings file. If not provided will look for a default path in ~/.config/kindle2doc/kindle_clippings_path
  -o, --one-file
          Sets if the highlights are exported to one or many files
  -h, --help
          Print help
  -V, --version
          Print version
```

Provide the path to `My Clippings.txt` in the command or set it in a file at `~/.config/kindle2doc/kindle_clippings_path`.

An example `kindle_clippings_path`:
```
/run/media/tarik/Kindle/documents/My Clippings.txt
```

## Installation
Clone the repo
```bash
https://github.com/tarikjaber/kindle2doc.git
```

Build the program
```bash
cargo build --release
```

Move the executable to a folder in `$PATH`
```bash
mv target/release/kindle2org ~/bin/kindle2org
```
