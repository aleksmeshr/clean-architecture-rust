pub mod presentation;
pub mod usecases;
pub mod utils;

extern crate dotenv;
extern crate log;
extern crate diesel;
extern crate r2d2;

use std::{env, net::TcpListener};

use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};
use animal_facts_repositories::http::cat_facts_repository::CatFactsRepository;
use animal_facts_repositories::http::connection::HttpConnection;
use animal_facts_repositories::postgres::db_connection::DbConnection;
use animal_facts_repositories::postgres::dog_facts_repository::DogFactsRepository;

use crate::presentation::shared::app_state::AppState;
use crate::presentation::routes;

pub fn serve(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::try_init();

    let db_connection = DbConnection { db_name: db_name.to_string() };
    let http_connection = HttpConnection {};

    let data = web::Data::new(AppState {
        app_name: String::from("Animal Facts API"),
        cats_repository: CatFactsRepository {
            http_connection,
            source: dotenv::var("CATS_SOURCE").expect("CATS_SOURCE must be set"),
        },
        dogs_repository: DogFactsRepository { db_connection },
    });

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .configure(routes::routes)
    })
    .listen(listener)?
    .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}
