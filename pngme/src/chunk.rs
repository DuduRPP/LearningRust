use std::fmt::Display;

use anyhow::{anyhow, Result};
use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len() as u32;

        let data_for_crc = &[&chunk_type.bytes(), data.as_slice()].concat();
        let crc_algo: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc = crc_algo.checksum(data_for_crc);

        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    fn crc(&self) -> u32 {
        self.crc
    }
    fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn data_as_string(&self) -> Result<String> {
        let data = String::from_utf8_lossy(self.data());
        Ok(data.to_string())
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.length().to_be_bytes().iter());
        bytes.extend(self.chunk_type().bytes().iter());
        bytes.extend_from_slice(self.data());
        bytes.extend(self.crc().to_be_bytes().iter());
        bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        let mut length: [u8; 4] = [0, 0, 0, 0];
        for i in 0..4 {
            length[i] = value[i];
        }
        let length: u32 = ((length[0] as u32) << 24)
            | ((length[1] as u32) << 16)
            | ((length[2] as u32) << 8)
            | length[3] as u32;
        let length: usize = length as usize;

        let mut chunk_type = [0, 0, 0, 0];
        for i in 4..8 {
            chunk_type[i - 4] = value[i];
        }
        let chunk_type = ChunkType::try_from(chunk_type)?;

        let mut data: Vec<u8> = Vec::new();
        for i in 8..length + 8 {
            data.push(value[i]);
        }

        let generated_chunk = Chunk::new(chunk_type, data);

        // Verifying crc
        let mut png_crc = [0, 0, 0, 0];
        for i in length + 8..length + 12 {
            png_crc[i - length - 8] = value[i];
        }
        let png_crc: u32 = u32::from_be_bytes(png_crc);
        println!("{png_crc}");
        if png_crc == generated_chunk.crc() {
            Ok(generated_chunk)
        } else {
            Err(anyhow!("Invalid CRC"))
        }
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Length:{}\nType:{}\nData:{}\nCRC:{}",
            self.length(),
            self.chunk_type().to_string(),
            self.data_as_string().unwrap_or_else(|_|{
                "couldn't read data as string".to_string()
            }),
            self.crc(),
        )
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
        println!("{:?}", chunk_type.bytes());
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
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
