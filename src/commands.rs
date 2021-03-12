use std::convert::TryFrom;
use std::fs::read;
use std::path::Path;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

pub fn encode<P: AsRef<Path>>(file_path: P, chunk_type: &str, data: &str) -> Result<()> {
    let file = read(file_path)?;
    let mut png = Png::try_from(&file[..])?;
    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunk = Chunk::new(chunk_type, data.as_bytes().to_vec());

    png.append_chunk(chunk);

    Ok(())
}
