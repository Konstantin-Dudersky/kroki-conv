use clap::{Parser, Subcommand, command};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets a kroki url
    ///
    /// Examples:
    ///
    /// - https://kroki.io
    ///
    /// - http://localhost:8000
    #[arg(short, long, default_value = "http://localhost:8000")]
    pub url: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(PartialEq, Subcommand)]
pub enum Commands {
    /// Compile all diagrams and end
    Compile,

    /// Compile all diagrams and watch for changes
    Watch,
}
