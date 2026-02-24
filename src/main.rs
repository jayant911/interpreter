#![allow(unused_variables)]
use interpreter::*;
use miette::{IntoDiagnostic, WrapErr};
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// does tokenize
    Tokenize { filename: PathBuf },
}

fn main() -> miette::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Tokenize { filename } => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(&filename)
                .into_diagnostic()
                .wrap_err_with(|| format!("reading '{}' failed", filename.display()))?;

            let lexer = Lexer::new(&file_contents);
            for token in lexer {
                let token = token?;
                println!("{token}");
            }
            println!("EOF null");
        }
    }
    Ok(())
}
