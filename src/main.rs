use actix_web::{App, HttpServer};
use mvc_like::routes;

const SERVER_ADDR: &str = "127.0.0.1:8000";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("[SERVER] http://{}/", SERVER_ADDR);
    HttpServer::new(|| App::new().configure(routes::routes))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
