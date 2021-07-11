use actix_web::{get, web, HttpResponse, Responder, Scope};

pub fn get_routes_calc() -> Scope {
    web::scope("/calc").service(get_calc)
}

#[get("")]
async fn get_calc() -> impl Responder {
    HttpResponse::Ok().body("5 + 5 = 10")
}
