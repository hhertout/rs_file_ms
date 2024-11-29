use std::io::{Error, ErrorKind};

use crate::mime::MimeType;

pub fn check_magic_number(file_content: &[u8]) -> Result<MimeType, Error> {
    match file_content {
        [0xFF, 0xD8, 0xFF, ..] => Ok(MimeType::JPG),
        [0x89, 0x50, 0x4E, 0x47, ..] => Ok(MimeType::PNG),
        [b'%', b'P', b'D', b'F', ..] => Ok(MimeType::PDF),
        [b'R', b'I', b'F', b'F', ..] if file_content.len() >= 12 && &file_content[8..12] == b"WEBP" => {
            Ok(MimeType::WEBP)
        }
        [b'G', b'I', b'F', ..] => Ok(MimeType::GIF),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Unsupported mimetype")),
    }
}

pub fn check_mime_type_from(expected: MimeType, file_content: &[u8]) -> Result<MimeType, Error> {
    let mime = check_magic_number(file_content)?;
    log::debug!("Mime type of file is {:?}", mime);

    if mime != expected {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid mime type"));
    }

    Ok(mime)
}

pub fn check_mime_type_from_vec(
    mime_types: Vec<MimeType>,
    file_content: &[u8],
) -> Result<MimeType, Error> {
    let mime = check_magic_number(file_content)?;

    if !mime_types.contains(&mime) {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid mime type"));
    }

    Ok(mime)
}
