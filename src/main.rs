mod routes;

use actix_web::{App, HttpServer};
use routes::get_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_routes()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
