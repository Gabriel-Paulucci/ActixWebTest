mod calc;

use actix_web::{web, Scope};

use self::calc::get_routes_calc;

pub fn get_routes() -> Scope {
    web::scope("/").service(get_routes_calc())
}
