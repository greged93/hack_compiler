use std::path::PathBuf;

use clap::{command, Parser};
use compiler::tokenizer::JackTokenizer;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional path to a file or a directory
    #[arg(short, long)]
    path: Option<PathBuf>,
}

fn main() {
    let path = Args::parse().path.unwrap_or_else(|| PathBuf::from("."));
    let jack_files = WalkDir::new(path)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .map(|ext| ext == "jack")
                    .unwrap_or_default()
        })
        .map(|entry| entry.path().to_path_buf())
        .collect::<Vec<_>>();

    for j in jack_files {
        let name = j
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string()
            + "Compiler.xml";
        let mut output_path = j.clone();
        output_path.set_file_name(name);

        let mut tokenizer = JackTokenizer::new(j);
        let mut acc = String::new();
        acc += "<tokens>\n";
        while tokenizer.has_more_tokens() {
            let token = tokenizer.current_token();
            acc += &(token.start_xml() + " " + &token.to_xml() + " " + &token.end_xml() + "\n");
            tokenizer.advance();
        }
        acc += r"</tokens>";

        std::fs::write(output_path, acc).expect("failed to write output");
    }
}
