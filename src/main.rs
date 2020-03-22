use actix_web::{App, HttpServer};
use listenfd::ListenFd;
mod food;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
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
