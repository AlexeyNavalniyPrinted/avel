use std::error::Error;

use actix_web::{get, HttpResponse, post, Responder};

pub type ResponseResult = Result<HttpResponse, Box<dyn Error>>;

#[post("/new")]
pub async fn new_short_link() -> ResponseResult  {
    Ok(HttpResponse::Ok().body("good"))
}




#[get("/")]
pub async fn hello() -> impl Responder {
    "Hello World!".to_string()
}