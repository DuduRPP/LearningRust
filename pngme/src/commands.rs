use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use crate::Result;
use crate::EncodeArgs;
use crate::args::DecodeArgs;
use crate::args::PrintArgs;
use crate::args::RemoveArgs;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn encode(args: EncodeArgs) -> Result<()>{
    let add_header = !Path::new(&args.file_path).exists();

    let mut f = fs::OpenOptions::new().write(true).create(true).read(true).open(args.file_path)?;
    let mut buffer: Vec<u8> = Vec::new();

    f.read_to_end(&mut buffer)?;
    
    // let mut png = Png::try_from(buffer.as_slice())?;
    
    if add_header{
        f.write(&Png::STANDARD_HEADER)?;
    }

    let chunk_type: ChunkType = ChunkType::from_str(&args.chunk_type)?;
    let chunk: Chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    // png.append_chunk(chunk);
    
    f.write_all(chunk.as_bytes().as_slice())?;

    println!("Message encoded successfully!");

    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()>{
    let mut f = fs::File::open(args.file_path)?;
    let mut buffer: Vec<u8> = Vec::new();

    f.read_to_end(&mut buffer)?;
    let png = Png::try_from(buffer.as_slice())?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type){
        println!("{}",chunk);
    }

    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()>{
    let mut f = fs::OpenOptions::new().read(true).open(&args.file_path)?;
    let mut buffer: Vec<u8> = Vec::new();

    f.read_to_end(&mut buffer)?;
    
    let mut png = Png::try_from(buffer.as_slice())?;
    let chunk = png.remove_chunk(&args.chunk_type)?;

    let mut f = fs::OpenOptions::new().write(true).truncate(true).open(&args.file_path)?;
    f.write_all(png.as_bytes().as_slice())?;

    println!("{}",chunk);

    Ok(())
}

pub fn print(args: PrintArgs) -> Result<()>{

    let mut f = fs::File::open(args.file_path)?;
    let mut buffer: Vec<u8> = Vec::new();

    f.read_to_end(&mut buffer)?;
    let png = Png::try_from(buffer.as_slice())?;

    println!("{}",png);
    Ok(())
}
