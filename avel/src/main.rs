mod fns;

use std::env;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use crate::fns::{hello, short_link, save_short_link};

pub struct AppState {
    pub cockroachdb_session: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env::set_var("RUST_LOG", "debug");
    env_logger::init();



    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(short_link)
            .service(save_short_link)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
