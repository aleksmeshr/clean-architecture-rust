use actix_web::web;

use crate::presentation::controllers::cat_fact_controllers;
use crate::presentation::controllers::dog_facts_controllers;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api/v1/dogs").configure(dog_facts_controllers::routes))
        .service(web::scope("/api/v1/cats").configure(cat_fact_controllers::routes));
}
