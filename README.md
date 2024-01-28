## kindle2doc
Converts kindle highlights into org, markdown, or text files.

## Usage
```
Usage: kindle2org [OPTIONS]

Options:
  -o, --output-file <OUTPUT_FILE>
          Path to output file [default: highlights.org]
  -e, --export-type <EXPORT_TYPE>
          Export type. Options are 'org', 'md', and 'txt' [default: org]
  -c, --clippings-path <CLIPPINGS_PATH>
          Path to Kindle clippings file
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
