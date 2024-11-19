use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{http::StatusCode, post, HttpResponse, Responder};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

#[post("/file")]
pub async fn post_file(MultipartForm(form): MultipartForm<UploadForm>) -> impl Responder {
    log::debug!(
        "File form data handled => name='{}'; content-type={:?}; size: {}bytes;",
        form.file.file_name.unwrap_or_else(|| "unknow".to_string()),
        form.file.content_type,
        form.file.size
    );
    HttpResponse::new(StatusCode::OK)
}
