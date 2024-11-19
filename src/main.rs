use actix_web::{App, HttpServer};
use domain::controller::{ping, v1::get_v1_service};

mod domain;
mod infra;

pub extern crate file_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("4000"));
    let ipv4 = "0.0.0.0";

    log::info!("📡 Server starting ! Listening on {}:{}", ipv4, port);

    HttpServer::new(|| App::new().service(ping).service(get_v1_service()))
        .bind((
            "127.0.0.1",
            port.parse::<u16>().expect("Port cannot be parsed"),
        ))?
        .run()
        .await
}
