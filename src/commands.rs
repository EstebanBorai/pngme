use std::fs::{read, File};
use std::path::PathBuf;
use std::str::FromStr;
use std::{convert::TryFrom, io::Write};

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::error::PngMeError;
use crate::png::Png;
use crate::Result;

pub fn encode(encode_args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(encode_args.file_path)?;
    let chunk_type = ChunkType::from_str(encode_args.chunk_type.as_str())?;
    let chunk = Chunk::new(chunk_type, encode_args.message.as_bytes().to_vec());
    let output_file_path = if let Some(output_file_name) = encode_args.output_file {
        output_file_name
    } else {
        PathBuf::from_str("output.png").unwrap()
    };

    png.append_chunk(chunk);
    png.write_file(output_file_path)?;

    Ok(())
}

pub fn decode(decode_args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(decode_args.file_path)?;

    if let Some(chunk) = png.chunk_by_type(decode_args.chunk_type.as_str()) {
        println!("{}", chunk);
    } else {
        eprintln!("Chunk type: {} not found", decode_args.chunk_type);
    }

    Ok(())
}

pub fn remove(remove_args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(remove_args.file_path.clone())?;

    png.remove_chunk(remove_args.chunk_type.as_str())?;
    png.write_file(remove_args.file_path)
}

pub fn print(print_args: PrintArgs) -> Result<()> {
    let png = Png::from_file(print_args.file_path)?;

    for chunk in png.chunks().iter() {
        println!("{}", chunk);
    }

    Ok(())
}
