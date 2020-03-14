use actix_web::{get, HttpResponse, Responder};

pub struct Food {
    name: String,
}

#[get("/food")]
async fn foods() -> impl Responder {
    HttpResponse::Ok().body("asdf")
}
