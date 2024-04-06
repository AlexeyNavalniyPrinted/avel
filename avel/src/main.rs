mod fns;

use std::env;
use std::error::Error;
use actix_web::{App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use sqlx::{Pool, Postgres, query};
use crate::fns::{hello, new_short_link};

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
            .service(new_short_link)
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