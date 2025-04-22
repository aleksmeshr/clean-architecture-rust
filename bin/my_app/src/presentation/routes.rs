use actix_web::web;

use crate::presentation::product::product_controller;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api/v1/product").configure(product_controller::routes))
}
