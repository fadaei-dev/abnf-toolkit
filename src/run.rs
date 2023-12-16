use crate::cli;
use crate::lexer::Lexer;
use clap::Parser;
use std::fs;

/// RUN
pub fn run() {
    let cli = cli::Cli::parse();

    if let Some(file_path) = cli.file {
        match fs::read_to_string(file_path) {
            Ok(source) => {
                let mut lexer = Lexer::new(&source);

                match lexer.tokenize() {
                    Ok(tokens) => {
                        for t in tokens {
                            println!("{t}");
                        }
                    }
                    Err(err) => {
                        println!("-The following syntax errors where found:-");
                        for e in err {
                            println!("{e}");
                        }
                    }
                }
            }
            Err(_err) => println!("Errors"),
        }
    }
}
