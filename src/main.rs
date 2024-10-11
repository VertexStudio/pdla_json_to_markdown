use std::{fs::OpenOptions, io::Write};

use json_to_markdown::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //Path to the input JSON file generated from pdf-document-layout-analysis
    #[arg(short, long)]
    input: String,

    //Path to the output markdown file
    #[arg(short, long)]
    output: String,
}

//usage: cargo run -- -i <path_to_json_from_pdf-document-layout-analysis> -o <path_to_output_md_file>

fn main() {
    let args = Args::parse();

    let json_data = std::fs::read_to_string(args.input).expect("Error reading file");

    let markdown_string = convert_json_to_markdown(&json_data).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}/readme_result.md", args.output))
        .expect("Unable to open file for writing");

    file.write_all(markdown_string.as_bytes())
        .expect("Unable to write to the file");
}
