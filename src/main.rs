mod routes;

use actix_web::{App, HttpServer};
use routes::config_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
