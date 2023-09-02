use core::fmt;
use std::str::FromStr;

use crate::error::PngMeError;

#[derive(PartialEq, Clone, Debug)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && Self::is_valid_byte_array(&self.bytes)
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_uppercase() || byte.is_ascii_lowercase()
    }

    fn is_valid_byte_array(arr: &[u8]) -> bool {
        if arr.len() != 4 {
            return false;
        }

        for byte in arr {
            if !Self::is_valid_byte(*byte) {
                return false;
            }
        }

        true
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = super::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if !ChunkType::is_valid_byte_array(&value) {
            return Err(Box::new(PngMeError::InvalidChunkType));
        }

        Ok(ChunkType { bytes: value })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.bytes {
            write!(f, "{}", byte as char)?;
        }
        Ok(())
    }
}

impl FromStr for ChunkType {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Box::new(PngMeError::InvalidChunkType));
        }

        let bytes: [u8; 4] = s[..4].as_bytes().try_into()?;
        let chunk_type: super::Result<ChunkType> = bytes.try_into();

        chunk_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let are_chunks_equal = chunk_type_1 == chunk_type_2;
        assert!(are_chunks_equal);
    }
}
