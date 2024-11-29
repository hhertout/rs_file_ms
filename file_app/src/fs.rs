use std::{
    fs::File,
    io::{Error, Write},
};
use uuid::Uuid;

pub fn save(path: &str, content: &[u8], extension: &str) -> Result<(), Error> {
    let filename = format!("{}/{}.{}", path, Uuid::new_v4(), extension);
    log::debug!("Saving file {}", filename);

    let mut file = File::create(&filename)?;
    let _ = file.write_all(content);

    Ok(())
}

pub fn get(_filename: &str) -> Result<Vec<u8>, Error> {
    let content: Vec<u8> = Vec::new();
    return Ok(content);
}
