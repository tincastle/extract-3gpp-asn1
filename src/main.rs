use clap::{CommandFactory, Parser};
use extract_3gpp_asn1::extract_asn1_blocks;
use std::io::{Read, IsTerminal};

#[derive(Parser)]
#[command(author, version, about, long_about = None, after_help = "Note: Input can be provided via a file path or standard input (stdin) with pipe (|).")]
struct Cli {
    /// The file to process
    #[arg(value_name = "FILE")]
    path: Option<std::path::PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let content = if !std::io::stdin().is_terminal() {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("could not read stdin");
        buffer
    } else if let Some(path) = cli.path {
        std::fs::read_to_string(path).expect("could not read file")
    } else {
        let _ = Cli::command().print_help();
        std::process::exit(1);
    };
    let extracted = extract_asn1_blocks(&content);
    println!("{}", extracted);
}