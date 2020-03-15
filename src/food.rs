use actix_web::{web, get, post, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Food {
    id: u64,
    name: String,
}

#[get("/food")]
async fn get_foods() -> impl Responder {
    web::Json(
        vec![
            Food { id: 0, name: "apple".to_owned()},
            Food { id: 1, name: "pear".to_owned()}
        ]
    )
}

#[get("/food/{id}")]
async fn get_food_by_id(path: web::Path<u64>) -> impl Responder {
    web::Json(Food { id: *path, name: "burger".to_owned() })
}

#[post("/food")]
async fn post_food(food: web::Json<Food>) -> impl Responder {
    String::from(&food.name)
}
