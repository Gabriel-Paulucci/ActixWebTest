mod calc;

use actix_web::web::ServiceConfig;

use self::calc::get_routes_calc;

pub fn config_routes(config: &mut ServiceConfig) {
    config.service(get_routes_calc());
}
