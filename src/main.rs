use std::str::FromStr;

use clap::Parser;
use pngme::{
    args::{Cli, Commands},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
    Result,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    return match &cli.command {
        Commands::Encode {
            png_file_path,
            chunk_type,
            message,
        } => {
            let mut png = Png::from_path(png_file_path)?;

            let chunk_type = ChunkType::from_str(&chunk_type)?;
            let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

            png.append_chunk(chunk);
            png.save_changes(png_file_path)?;

            Ok(())
        }
        Commands::Decode { png_file_path, chunk_type } => {
            let png: Png = Png::from_path(png_file_path)?;

            if let Some(data) = png.chunk_by_type(chunk_type) {
                println!("{:#?}", data.data_as_string());
                Ok(())
            } else {
                Err("message enot found".into())
            }
        }
        Commands::Remove { png_file_path, chunk_type } => {
            let mut png = Png::from_path(png_file_path)?;

            png.remove_chunk(chunk_type)?;
            png.save_changes(png_file_path)?;

            Ok(())
        }
        Commands::Print { path } => {
            let png = Png::from_path(path)?;

            println!("{:#?}", png.to_string());

            Ok(())
        }
    };
}
