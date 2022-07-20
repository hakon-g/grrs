// A tutorial to write a CLI app in Rust
// The grrs app is similar to grep

#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};
use log::{info, trace, warn};

//mod lib;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    trace!("Starting up...");
    
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file '{}'", &args.path.display()))?;
   
    grrs::find_matches(&content, &args.pattern, std::io::stdout());

    Ok(())
}