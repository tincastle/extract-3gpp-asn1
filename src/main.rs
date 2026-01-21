use clap::{CommandFactory, Parser};
use extract_3gpp_asn1::{
    TagStrategy, extract_asn1_blocks, remove_delimited_comments, remove_multiline_comments,
    remove_trailing_comments,
};
use std::io::{IsTerminal, Read};

#[derive(Parser)]
#[command(author, version, about, long_about = None, after_help = "Note: Input can be provided via a file path or standard input (stdin) with pipe (|).")]
struct Cli {
    /// The file to process
    #[arg(value_name = "FILE")]
    path: Option<std::path::PathBuf>,

    /// Comment process level
    /// - 0 (no option given): All comments are preserved
    /// - 1 (-c): Only need codes and conditions are preserved
    /// - 2 or higher (-cc or more): All comments are removed
    /// Note: This is not syntax-aware but simple pattern-matching
    #[arg(short, long, action = clap::ArgAction::Count, verbatim_doc_comment)]
    comment_process_level: u8,
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
    match cli.comment_process_level {
        0 => println!("{}", extracted),
        1 => {
            let multiline_removed = remove_multiline_comments(&extracted);
            let delimited_removed = remove_delimited_comments(&multiline_removed);
            let trailing_removed =
                remove_trailing_comments(&delimited_removed, TagStrategy::Preserve);
            println!("{}", trailing_removed);
        }
        _ => {
            let multiline_removed = remove_multiline_comments(&extracted);
            let delimited_removed = remove_delimited_comments(&multiline_removed);
            let trainling_removed =
                remove_trailing_comments(&delimited_removed, TagStrategy::Remove);
            println!("{}", trainling_removed);
        }
    }
}
