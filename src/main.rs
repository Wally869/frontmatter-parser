use clap::{Parser, Subcommand};
use frontmatter_parser::{parse_directory, parse_file};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "frontmatter-parser")]
#[command(about = "Parse frontmatter from markdown files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse frontmatter from a single file
    File {
        /// Path to the markdown file
        path: PathBuf,
    },
    /// Parse frontmatter from all markdown files in a directory
    Dir {
        /// Path to the directory
        path: PathBuf,

        /// Recursively search subdirectories
        #[arg(short, long, default_value_t = false)]
        recursive: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::File { path } => match parse_file(&path) {
            Ok(fm) => match fm.to_json() {
                Ok(json) => {
                    println!("{}", json);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Error serializing to JSON: {}", e);
                    ExitCode::FAILURE
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                ExitCode::FAILURE
            }
        },
        Commands::Dir { path, recursive } => {
            let results = parse_directory(&path, recursive);
            let mut has_errors = false;
            let mut frontmatters = Vec::new();

            for result in results {
                match result {
                    Ok(fm) => {
                        frontmatters.push(serde_json::json!({
                            "file": fm.path.display().to_string(),
                            "frontmatter": fm.data
                        }));
                    }
                    Err(e) => {
                        eprintln!("Warning: {}", e);
                        has_errors = true;
                    }
                }
            }

            match serde_json::to_string_pretty(&frontmatters) {
                Ok(json) => {
                    println!("{}", json);
                    if has_errors {
                        ExitCode::from(2) // Partial success
                    } else {
                        ExitCode::SUCCESS
                    }
                }
                Err(e) => {
                    eprintln!("Error serializing to JSON: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
    }
}
