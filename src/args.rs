use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap( value_parser)]
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encode a Message into a PNG file(
    Encode{
        png_file_path: PathBuf,
        chunk_type: String,
        message: String,
    },
    /// Decode message of a PNG file
    Decode{
        png_file_path: PathBuf,
        chunk_type: String,
    },
    /// Remove an encoded Message from a PNG file
    Remove {
        png_file_path: PathBuf,
        chunk_type: String,
    },
    /// Print PNG file information
    Print {
        path: PathBuf,
    },
}
