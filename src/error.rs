use std::fmt;

#[derive(Debug)]
pub enum PngMeError {
  InvalidChunkLength(String),
  InvalidBytes(String),
}

impl fmt::Display for PngMeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        PngMeError::InvalidChunkLength(chunk) => write!(f, "The chunk {} have a {} length, the max length is 4", chunk, chunk.len()),
        PngMeError::InvalidBytes(chunk) => write!(f, "The chunk {} has invalid bytes", chunk),
      }
  }
}
