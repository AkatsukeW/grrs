use std::sync::mpsc::Receiver;
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    let result = std::fs::read_to_string("test.txt");
    match result {
        Ok(content) => println!("{}", content),
        Err(error) => println!("{}", error),
    }
}

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

