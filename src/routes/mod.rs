mod calc;
mod count;

use actix_web::web::ServiceConfig;

use self::{calc::get_routes_calc, count::get_routes_count};

pub fn config_routes(config: &mut ServiceConfig) {
    config
        .service(get_routes_calc())
        .service(get_routes_count());
}
