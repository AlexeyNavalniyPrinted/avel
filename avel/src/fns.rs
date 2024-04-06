use std::error::Error;
use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::http::header;
use actix_web::web::Redirect;
use serde::{Deserialize, Serialize};
use serde::__private::from_utf8_lossy;
use url::{ParseError, Url};

#[derive(Serialize, Deserialize)]
struct LinkShort {
    link: String,
}

type ResponseResult = Result<HttpResponse, Box<dyn Error>>;
#[get("/short/{link}")]
async fn short_link(link: web::Path<String>) -> Redirect {
    let redirect_url = format!("https://{}", link);
    let link_s = LinkShort { link: redirect_url.to_string() };

    Redirect::to(link_s.link)
}

#[post("/q")]
async fn save_short_link(link: web::Json<LinkShort>) -> HttpResponse {

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

