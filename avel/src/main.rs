mod fns;

use std::env;
use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use sqlx::{PgPool, Pool, Postgres, query};
use log::info;
use crate::fns::{hello, save_short_link, short_link};


pub struct AppState {
    pub cockroachdb_session: CockroachDBSession
}

type CockroachDBSession = Pool<Postgres>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let conn_url = "postgresql://roach:password@cockroach-cockroachdb-public.default.svc.cluster.local:26257/defaultdb?sslmode=verify-full&sslrootcert=/certs/ca.crt";

    let connection: CockroachDBSession = PgPool::connect(&conn_url).await.unwrap();
    queries(&connection).await;

    let app_state = Data::new(Arc::new(AppState {
        cockroachdb_session: connection
    }));

    info!("Successfully started server on 0.0.0.0:5000");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(hello)
            .service(short_link)
            .service(save_short_link)
    })
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}


async fn queries(connection: &CockroachDBSession) {
    let b = query(
        "CREATE TABLE IF NOT EXISTS links(short_link TEXT, full_link TEXT, PRIMARY KEY(short_link, full_link));"
    );

    b.execute(connection).await.unwrap();
}