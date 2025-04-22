pub mod presentation;
pub mod usecases;
pub mod utils;

extern crate diesel;
extern crate dotenv;
extern crate log;
extern crate r2d2;

use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::sync::Arc;

use infra::comment::http::comment_http_client::CommentHttpClient;
use infra::http::http_connection::HttpConnection;
use infra::postgres::db_connection::DbConnection;
use infra::product::postgres::product_db_repository::ProductDbRepository;

use crate::presentation::routes;
use crate::presentation::shared::app_state::AppState;

use crate::usecases::get_product_usecase::GetProductUseCaseImpl;

pub fn serve(
    listener: TcpListener,
    db_host: &str,
    db_name: &str,
) -> Result<Server, std::io::Error> {
    //env::set_var("RUST_BACKTRACE", "1");
    //env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::try_init().unwrap();

    let db_connection =
        DbConnection { database_host: db_host.to_string(), db_name: db_name.to_string() };
    let http_connection = HttpConnection {};

    let product_repository = Arc::new(ProductDbRepository { db_connection });
    let comment_client = Arc::new(CommentHttpClient {
        http_connection,
        endpoint: dotenv::var("COMMENT_SVC").expect("COMMENT_SVC must be set"),
    });

    let data = web::Data::new(AppState {
        app_name: String::from("My API"),
        get_product_usecase: Arc::new(GetProductUseCaseImpl::new(
            product_repository.clone(),
            comment_client.clone(),
        )),
    });

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || {
        App::new().app_data(data.clone()).wrap(Logger::default()).configure(routes::routes)
    })
    .listen(listener)?
    .run();

    println!("Server running on port {}, db_name {}", port, db_name);
    Ok(server)
}
