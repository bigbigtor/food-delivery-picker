#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use listenfd::ListenFd;
use food_delivery_picker::*;

mod food;
mod food_repo;
mod models;

embed_migrations!("migrations");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let conn = establish_connection();
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new().service(food::get_foods).service(food::get_food_by_id).service(food::post_food)
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("0.0.0.0:3000")?
    };
    server.run().await
}
