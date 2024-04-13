use std::error::Error;
use std::sync::Arc;

use actix_web::{error, get, HttpResponse, post, Responder, web};
use actix_web::web::{Data, Redirect};
use rand::distributions::Alphanumeric;
use rand::Rng;
use redis::Commands;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use url::Url;

use crate::AppState;

#[derive(Serialize, Deserialize)]
struct LinkShort {
    link: String
}

#[get("/{link}")]
pub async fn short_link(link: web::Path<String>, app_state: Data<Arc<AppState>>) -> Result<Redirect, Box<dyn Error>> {

    let mut redis_connection = app_state.redis_connection.lock().await;

    match redis_connection.get::<&String, String>(&link.to_string()) {
        Ok(url) => {
            return Ok(Redirect::to(url))
        }
        Err(_) => {}
    };

    let c = "SELECT full_link FROM links WHERE short_link = $1";
    let rows = match sqlx::query(c)
        .bind(&link.as_str())
        .fetch_all(&app_state.cockroachdb_connection)
        .await {
        Ok(r) => r,
        Err(e) => {
            return Err(Box::new(error::ErrorServiceUnavailable("The server is unavailable")));
        }
    };

    if let Some(row) = rows.get(0) {
        if let Some(full_link) = row.get::<Option<String>, &str>("full_link") {
            redis_connection.set::<&String, &String, String>(&link.to_string(), &full_link).unwrap();
            redis_connection.expire::<&String, String>(&link.to_string(), 300).unwrap();

            let c = "UPDATE links SET last_accessed = NOW() WHERE link = $1";

            sqlx::query(c)
                .bind(link.as_str())
                .execute(&app_state.cockroachdb_connection)
                .await.unwrap();

            return Ok(Redirect::to(full_link));
        }
    }

    return Err(Box::new(error::ErrorNotFound(
        "Not found",
    )));
}


#[post("/new")]
pub async fn save_short_link(link: web::Json<LinkShort>, app_state: Data<Arc<AppState>>) -> HttpResponse {
    let url = match Url::parse(&link.link) {
        Ok(url) => url,
        Err(_) => return HttpResponse::BadRequest().body("Link is incorrect"),
    };

    let c = "SELECT short_link FROM links WHERE full_link = $1";
    let rows = match sqlx::query(c)
        .bind(url.as_str())
        .fetch_all(&app_state.cockroachdb_connection)
        .await {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Internal server error");
        }
    };

    if let Some(row) = rows.get(0) {
        if let Some(short_l) = row.get::<Option<&str>, &str>("short_link") {
            return HttpResponse::Ok().body(format!("Existing short link: {}", short_l));
        }
    }

    let short_url: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    let insert_query = "INSERT INTO links (short_link, full_link, last_accessed) VALUES ($1, $2, NOW())";
    match sqlx::query(insert_query)
        .bind(&short_url)
        .bind(url.as_str())
        .execute(&app_state.cockroachdb_connection)
        .await {
        Ok(_) => HttpResponse::Ok().body(format!("New short link created: {}", short_url)),
        Err(_) => {
            HttpResponse::InternalServerError().body("Failed to save short link")
        }
    }
}

#[get("/")]
pub async fn hello() -> impl Responder {
    "Hello World!".to_string()
}