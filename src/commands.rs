use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};
use anyhow::{bail, Result};
use std::{
    fs::{self, File},
    io::Read,
    path::Path,
    str::FromStr,
};

fn get_png_from_path(path: &str) -> Result<Png> {
    let mut f = File::open(path)?;
    let mut bytes = vec![];
    f.read_to_end(&mut bytes)?;
    Png::try_from(bytes.as_slice())
}

pub fn encode(path: &str, chunk_type: &str, message: &str) -> Result<()> {
    let mut png = get_png_from_path(path)?;
    let end = png.remove_chunk(ChunkType::IEND)?;
    png.append_chunk(Chunk::new(
        ChunkType::from_str(chunk_type)?,
        message.as_bytes().into(),
    ));
    png.append_chunk(end);
    fs::write(Path::new(path), png.as_bytes())?;
    println!("Message encoded");
    Ok(())
}

pub fn decode(path: &str, chunk_type: &str) -> Result<()> {
    let png = get_png_from_path(path)?;
    if let Some(target) = png.chunk_by_type(chunk_type) {
        println!("Hidden message is: {}", target.data_as_string()?);
    } else {
        bail!("Chunk `{}` not found at path `{}`", chunk_type, path);
    }
    Ok(())
}

pub fn remove(path: &str, chunk_type: &str) -> Result<()> {
    let mut png = get_png_from_path(path)?;
    png.remove_chunk(chunk_type)?;
    fs::write(Path::new(path), png.as_bytes())?;
    println!("Chunk `{}` removed", chunk_type);
    Ok(())
}

pub fn print(path: &str) -> Result<()> {
    let png = get_png_from_path(path)?;
    let chunk_types: Vec<String> = png
        .chunks()
        .iter()
        .map(|c| c.chunk_type().to_string())
        .collect();
    println!("The following chunks can be decoded:");
    for chunk in chunk_types {
        println!("{}", chunk);
    }
    Ok(())
}
