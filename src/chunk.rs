use crate::chunk_type::{ChunkType };
use std::{fmt::{Display, Formatter}, io::Read };
use crc;
use crate::error::Error;

#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
	pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = Chunk::check_crc(&chunk_type, &data);
        Chunk {
            chunk_type,
            data,
            crc,
        }
	}

    fn check_crc(chunk_type: &ChunkType, data: &Vec<u8>) -> u32 {
        let buffer: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(
                data.iter()
            )
            .copied()
            .collect();
        const X25: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let crc = X25.checksum(&buffer);
        crc
    }

    pub fn update_data(&mut self, data: Vec<u8>) {
        let crc = Chunk::check_crc(&self.chunk_type, &data);
        self.data = data;
        self.crc = crc;
    }

	pub fn length(&self) -> usize {
        self.data.len()
	}

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn data_as_string(&self) -> Result<String, Error> {
        let data = std::str::from_utf8(&self.data)
            .map_err(|err| 
                Error::ChunkDataNotUtf8 { bytes: self.data.clone(), err }
            )?;
        Ok(String::from(data))
    }

    #[allow(dead_code)]
    pub fn crc(&self) -> u32 {
        self.crc
    }

    fn fix_4_bytes(&self, bytes: &[u8]) -> [u8;4] {
        let mut result = [0u8;4];
        let max_index = bytes.len() - 1;
        let mut index = max_index;
        loop {
            result[3 - (max_index - index)] = bytes[index];
            if index == 0 {
                break;
            }
            index -= 1;
        }
        result
    }

    fn size_as_bytes(&self) -> [u8;4] {
        let bytes = (self.length() as u32).to_be_bytes();
        self.fix_4_bytes(&bytes)
    }

    fn crc_as_bytes(&self) -> [u8;4] {
        let bytes = self.crc.to_be_bytes();
        self.fix_4_bytes(&bytes)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.size_as_bytes().iter()
            .chain(self.chunk_type.bytes())
            .chain(self.data.clone().iter())
            .chain(self.crc_as_bytes().iter())
            .copied()
            .collect()
    }
}

impl Display for Chunk {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.chunk_type(), self.data_as_string())
	}
}

enum ChunkPart {
    Type,
    Crc,
    Size,
}

fn get_4_bytes(bytes: &mut Box<&[u8]>, part: ChunkPart) -> Result<[u8;4], Error>{
    let mut buffer = [0u8; 4];
    bytes.read_exact(&mut buffer).map_err(|x| match part {
        ChunkPart::Size => Error::ChunkSizeReadError(x),
        ChunkPart::Type => Error::ChunkTypeReadError(x),
        ChunkPart::Crc => Error::ChunkCrcReadError(x),
    })?;
    Ok(buffer)
}

impl TryFrom<(usize, &Vec<u8>)> for Chunk {
    type Error = Error;
    fn try_from(value: (usize, &Vec<u8>)) -> Result<Chunk, Self::Error> {
        let (size, value) = value;
        let mut bytes : Box<&[u8]> = Box::new(value.as_ref()); 
        let chunk_type = ChunkType::try_from(get_4_bytes(&mut bytes, ChunkPart::Type)?)?;
        let mut data = vec![0u8;size];
        bytes.read_exact(&mut data).map_err(|x| Error::ChunkDataReadError(x))?;
        let b = get_4_bytes(&mut bytes, ChunkPart::Crc)?;
        let crc = u32::from_be_bytes(b);

        let chunk = Chunk::new(chunk_type, data);
        if chunk.crc != crc {
            Err(Error::ChunkCrcMismatch { expect: crc, actual: chunk.crc })
        } else {
            Ok(chunk)
        }
    }
}

impl TryFrom<&Vec<u8>> for Chunk {
	type Error = Error;
	fn try_from(value: &Vec<u8>) -> Result<Chunk, Self::Error> {
        let mut bytes = Box::new(value.as_ref());
        let size = u32::from_be_bytes(get_4_bytes(&mut bytes, ChunkPart::Size)?) as usize;
        Chunk::try_from((size, &bytes.to_vec())) 
	}	
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_update_data() {
        let mut chunk = testing_chunk();
        let crc = chunk.crc;
        chunk.update_data(chunk.data().to_owned());
        assert_eq!(chunk.crc, crc);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}