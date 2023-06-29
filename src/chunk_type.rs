
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::str::FromStr;
use std::convert::TryFrom;
use std::str;
use crate::error::Error;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ChunkType {
	bytes: [u8;4]
}

impl ChunkType {
	pub fn bytes(&self) -> &[u8] {
		&self.bytes[..]
	}

    #[allow(dead_code)]
	pub fn is_critical(&self) -> bool {
		let first = self.bytes[0];
		first.is_ascii_uppercase()
	}

    #[allow(dead_code)]
	pub fn is_public(&self) -> bool {
		let second = self.bytes[1];
		second.is_ascii_uppercase()
	}

    #[allow(dead_code)]
	pub fn is_reserved_bit_valid(&self) -> bool {
		let thrid = self.bytes[2]; 
		thrid.is_ascii_uppercase()
	}

    #[allow(dead_code)]
	pub fn is_safe_to_copy(&self) -> bool {
		let fourth = self.bytes[3];
		fourth.is_ascii_lowercase()
	}

    #[allow(dead_code)]
	pub fn is_valid(&self) -> bool {
		self.is_reserved_bit_valid()
	}

    pub fn to_str(&self) -> &str {
        std::str::from_utf8(&self.bytes).unwrap()
    }
}

impl PartialEq<str> for ChunkType {
    fn eq(&self, other: &str) -> bool {
        self.bytes() == other.as_bytes()
    }
}

impl Display for ChunkType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let not_valid = self.bytes.iter().find(|x| !x.is_ascii()).is_some();
		if not_valid {
			write!(f, "{} {} {} {}", self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3])
		} else {
			if let Ok(string) = String::from_utf8(self.bytes().to_owned()) {
				write!(f, "{}", string)
			} else {
				Err(std::fmt::Error)
			}
		}
	}
}

impl FromStr for ChunkType {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().into_iter().find(|x| !x.is_alphabetic()).is_some() {
            return Err(Error::ChunkTypeNotAplhaBetic{ string: String::from(s)})
        }

        ChunkType::try_from(s.as_bytes())
	}
}

impl TryFrom<&Vec<u8>> for ChunkType {
	type Error = Error;
	fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let slice = value.as_slice();
        ChunkType::try_from(slice) 
	}	
}

impl TryFrom<&[u8]> for ChunkType {
	type Error = Error;
	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            return Err(Error::ChunkTypeLenMismatch { bytes: Vec::from(value) })
        }
        let mut array:[u8;4] = [0;4];
        if let Err(e) = value.clone().read_exact(&mut array) {
            return Err(Error::ReadErr(e));
        }
        ChunkType::try_from(array)
	}	
}

impl TryFrom<[u8;4]> for ChunkType {
	type Error = Error;
	fn try_from(array: [u8;4]) -> Result<Self, Self::Error> {
        if array.iter().find(|x| !x.is_ascii_alphabetic()).is_some() {
            match std::str::from_utf8(&array) {
                Ok(string) => {
                    return Err(Error::ChunkTypeNotAplhaBetic { string: String::from(string) })
                },
                Err(err) => {
                    return Err(Error::ChunkTypeNotUtf8 { bytes: array, err, })
                }
            }
		}

        Ok(ChunkType { bytes: array })
	}	
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

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
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}