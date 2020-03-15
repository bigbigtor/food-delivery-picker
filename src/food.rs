use actix_web::{web, get, post, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Food {
    name: String,
}

#[get("/food")]
async fn get_foods() -> impl Responder {
    web::Json(
        vec![
            Food { name: "apple".to_owned()},
            Food { name: "pear".to_owned()}
        ]
    )
}

#[post("/food")]
async fn post_food(food: web::Json<Food>) -> impl Responder {
    String::from(&food.name)
}
