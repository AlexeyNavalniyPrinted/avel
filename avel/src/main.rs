mod fns;

use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use sqlx::{PgPool, Pool, Postgres};
use log::info;
use redis::Connection;
use tokio::sync::Mutex;
use crate::fns::{hello, save_short_link, short_link};


pub struct AppState {
    pub cockroachdb_connection: CockroachDBSession,
    pub redis_connection: RedisConnection
}

type CockroachDBSession = Pool<Postgres>;
type RedisConnection = Mutex<Connection>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut redis_password = String::new();

    File::open("/etc/redis/redis-password").unwrap().read_to_string(&mut redis_password).unwrap();

    let cockroach_con_url = "postgresql://roach:password@cockroach-cockroachdb-public.default.svc.cluster.local:26257/defaultdb?sslmode=verify-full&sslrootcert=/certs/ca.crt";

    let cockroachdb_connection: CockroachDBSession = PgPool::connect(&cockroach_con_url).await.unwrap();

    info!("Successfully connected to cockroachdb");

    let redis_client = redis::Client::open(format!("redis://:{}@redis-master.default.svc.cluster.local/", redis_password)).unwrap();
    let redis_connection = redis_client.get_connection().unwrap();

    info!("Successfully connected to redis");

    let app_state = Data::new(Arc::new(AppState {
        cockroachdb_connection,
        redis_connection: Mutex::new(redis_connection)
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


