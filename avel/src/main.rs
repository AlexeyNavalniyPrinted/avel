mod fns;

use std::env;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::{Pool, Postgres, query};
use crate::fns::{hello, save_short_link, short_link};

pub struct AppState {
    pub cockroachdb_session: CockroachDBSession
}

type CockroachDBSession = Pool<Postgres>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let conn_url = "postgresql://root@localhost:26257/defaultdb?sslmode=disable";

    let connection: CockroachDBSession = sqlx::PgPool::connect(&conn_url).await.unwrap();

    queries(&connection).await;

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


async fn queries(connection: &CockroachDBSession) {
    let b = query(
        "CREATE TABLE IF NOT EXISTS pen(c Text);"
    );

    b.execute(connection).await.unwrap();
}