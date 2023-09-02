use std::fmt;

#[derive(Debug)]
pub enum PngMeError {
    InvalidChunkLength(String),
    InvalidChunkType,
    InvalidCRC,
    InvalidPNGFileHeader,
    UnexistentChunkType,
}

impl fmt::Display for PngMeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PngMeError::InvalidChunkLength(chunk) => write!(
                f,
                "The chunk {} have a {} length, the max length is 4",
                chunk,
                chunk.len()
            ),
            PngMeError::InvalidChunkType => write!(f, "Invalid chunk type provided"),
            PngMeError::InvalidCRC => write!(f, "CRC IEEE Checksum didn't matched"),
            PngMeError::InvalidPNGFileHeader => write!(f, "Invalid PNG file header"),
            PngMeError::UnexistentChunkType => write!(f, "The provided chunk type doesn't exists"),
        }
    }
}

impl std::error::Error for PngMeError {}
