use std::fmt;

#[derive(Debug)]
pub enum PngMeError {
    InvalidChunkLength(String),
    InvalidBytes(String),
    InvalidChunkType,
    InvalidCRC,
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
            PngMeError::InvalidBytes(chunk) => write!(f, "The chunk {} has invalid bytes", chunk),
            PngMeError::InvalidChunkType => write!(f, "Invalid chunk type provided"),
            PngMeError::InvalidCRC => write!(f, "CRC IEEE Checksum didn't matched"),
        }
    }
}
