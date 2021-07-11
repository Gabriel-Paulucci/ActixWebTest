use std::sync::Mutex;

use actix_web::{
    get, post,
    web::{self, Data, Path},
    HttpResponse, Responder, Scope,
};

struct Counter {
    number: Mutex<i32>,
}

pub fn get_routes_count() -> Scope {
    let counter = Data::new(Counter {
        number: Mutex::new(0),
    });

    web::scope("/count")
        .service(get_count)
        .service(post_count)
        .app_data(counter)
}

#[get("")]
async fn get_count(data: Data<Counter>) -> impl Responder {
    let number = data.number.lock().unwrap();
    HttpResponse::Ok().body(format!("{}", number))
}

#[post("/{number_add}")]
async fn post_count(Path(number_add): Path<i32>, data: Data<Counter>) -> impl Responder {
    let mut number = data.number.lock().unwrap();
    *number += number_add;

    HttpResponse::Ok().body(format!("{}", number))
}
