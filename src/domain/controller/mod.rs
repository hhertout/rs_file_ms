use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

pub mod v1;

#[derive(Serialize)]
struct PingResponse {
    message: String
}

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(PingResponse {
        message: String::from("I'm alive")
    })
}