use clap::Parser;
use std::path::PathBuf;

// CLI TODO: Move to own module
#[derive(Parser)]
#[command(name = "ABNF toolkit")]
#[command(author = "Arad Fadaei")]
#[command(version = "0.1.0")]
#[command(about = "ABNF grammar toolkit", long_about = None)]
pub struct Cli {
    /// path to abnf file
    pub file: Option<PathBuf>,
}
