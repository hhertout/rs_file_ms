use std::io::Read;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{http::StatusCode, post, HttpResponse, Responder};
use file_app::{
    inspector::check_mime_type_from,
    mime::MimeType,
    resizer::{resize, ResizeMode},
};
use serde::Serialize;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "200MB")]
    file: TempFile,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    _type: String,
    error: String,
}

#[post("/file")]
pub async fn post_file(MultipartForm(mut form): MultipartForm<UploadForm>) -> impl Responder {
    log::debug!(
        "File form data handled => name='{}'; content-type={:?}; size: {}bytes;",
        form.file.file_name.unwrap_or_else(|| "unknown".to_string()),
        form.file.content_type,
        form.file.size
    );

    let mut content = Vec::new();
    if let Err(err) = form.file.file.read_to_end(&mut content) {
        log::error!("Fail to read file {:?}", err);
        return HttpResponse::InternalServerError().body("Error reading file content");
    };

    match check_mime_type_from(MimeType::JPG, &content) {
        Ok(_) => {}
        Err(err) => {
            log::error!("Mime type check failed: {:?}", err);
            return HttpResponse::BadRequest().json(ErrorResponse {
                _type: err.kind().to_string(),
                error: err.to_string(),
            });
        }
    };

    let _ = resize(ResizeMode::Interpolated, &content);
    // let _ = resize(ResizeMode::Nearest, &content);

    //let _ = save("temp", &content, &MimeType::JPG.to_string());

    HttpResponse::new(StatusCode::OK)
}
