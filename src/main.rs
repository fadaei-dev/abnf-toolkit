use clap::Parser;
use parser::lexer::Lexer;
use std::{fs, io, path::PathBuf};

mod parser;
mod position;
mod report;

// CLI TODO: Move to own module
#[derive(Parser)]
#[command(name = "ABNF toolkit")]
#[command(author = "Arad Fadaei")]
#[command(version = "0.1.0")]
#[command(about = "ABNF grammar toolkit", long_about = None)]
struct Cli {
    /// path to abnf file
    file: Option<PathBuf>,
}

fn read_to_lexer(file: PathBuf) -> io::Result<Lexer> {
    let source = fs::read_to_string(file)?;
    Ok(Lexer::new(source))
}

fn main() {
    let cli = Cli::parse();

    if let Some(file_path) = cli.file {
        match read_to_lexer(file_path) {
            Ok(mut lexer) => match lexer.scan_tokens() {
                Ok(_tokens) => println!("Tokens"),
                Err(_err) => println!("Errors"),
            },
            Err(err) => println!("{err}"),
        }
    }
}
