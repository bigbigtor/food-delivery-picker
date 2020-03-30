use actix_web::{web, get, post, Responder};
use crate::models::NewFood;
use crate::food_repo;

#[get("/food")]
async fn get_foods() -> impl Responder {
    web::Json(food_repo::get_foods())
}

#[get("/food/{id}")]
async fn get_food_by_id(id: web::Path<i32>) -> impl Responder {
    web::Json(food_repo::get_food_by_id(*id))
}

#[post("/food")]
async fn post_food(food: web::Json<NewFood>) -> impl Responder {
    web::Json(food_repo::save_food(&*food))
}
