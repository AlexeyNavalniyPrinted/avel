use std::error::Error;
use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::web::Redirect;
use serde::{Deserialize, Serialize};
use url::Url;


pub async fn new_short_link() -> HttpResponse  {
    HttpResponse::Ok().body("good")
}

#[derive(Serialize, Deserialize)]
struct LinkShort {
    link: String
}

#[get("/short/{link}")]
pub async fn short_link(link: web::Path<String>) -> Redirect {
    let redirect_url = format!("https://{}", link);
    let link_s = LinkShort { link: redirect_url.to_string() };

    Redirect::to(link_s.link)
}

#[post("/q")]
pub async fn save_short_link(link: web::Json<LinkShort>) -> HttpResponse {
    let _url = match Url::parse(&link.link) {
        Ok(arg) => {
            arg
        }
        Err(_) => {
            return HttpResponse::BadRequest().body("Adolf Hitler kill niggers")
        }
    };
    HttpResponse::Ok().json(link)
}

#[get("/")]
pub async fn hello() -> impl Responder {
    "Hello World!".to_string()
}