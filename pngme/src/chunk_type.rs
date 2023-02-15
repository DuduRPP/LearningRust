use std::{str::FromStr, fmt::Display};
use anyhow::{anyhow,Result};

#[derive(PartialEq, Debug)]
pub struct ChunkType{
    byte_type: [u8;4],
}

impl ChunkType{
    pub fn bytes(&self) -> [u8;4]{
        let byte_type: [u8;4] = self.byte_type.map(|x| x as u8);
        byte_type
    }
    fn is_critical(&self) -> bool{
        self.byte_type[0] & 0b100000 == 0
    }
    fn is_public(&self) -> bool {
        self.byte_type[1] & 0b100000 == 0
    }
    
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.byte_type[2] & 0b100000 == 0
    }

    fn is_safe_to_copy(&self) -> bool {
        self.byte_type[3] & 0b100000 != 0
    }
}

impl TryFrom<[u8;4]> for ChunkType{
    type Error = anyhow::Error;
    fn try_from(value: [u8;4]) -> Result<Self, Self::Error>{
        Ok(ChunkType{byte_type: value})
    }
}
impl FromStr for ChunkType{
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4{
            Err(anyhow!("type should be of 4 bytes"))
        } else{
            let mut full_letters = true;
            let byte_type: [u8;4] = s.as_bytes().try_into().unwrap();

            let byte_type: [u8;4] = byte_type.map(|x| if full_letters && !x.is_ascii_alphabetic(){
                full_letters = false;
                x
            }else {
                x
            });

            if full_letters == true{
                Ok(ChunkType { byte_type }) 
            } else {
                Err(anyhow!("invalid chunk type byte"))
            }

        }
    }
}
impl Display for ChunkType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f,"{}",std::str::from_utf8(&self.byte_type).unwrap())
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

