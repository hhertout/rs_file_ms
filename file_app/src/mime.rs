use std::fmt::{self, Debug};

#[derive(PartialEq)]
pub enum MimeType {
    JPG,
    PNG,
    GIF,
    WEBP,
    PDF,
}

impl ToString for MimeType {
    fn to_string(&self) -> String {
        match self {
            MimeType::JPG => String::from("jpg"),
            MimeType::PNG => String::from("png"),
            MimeType::GIF => String::from("gif"),
            MimeType::WEBP => String::from("webp"),
            MimeType::PDF => String::from("pdf"),
        }
    }
}

impl Debug for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mime type = {}", self.to_string())
    }
}
