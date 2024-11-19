use actix_web::{web, Scope};
use file::post_file;

pub(crate) mod file;

pub fn get_v1_service() -> Scope {
    web::scope("/api/v1").service(post_file)
}