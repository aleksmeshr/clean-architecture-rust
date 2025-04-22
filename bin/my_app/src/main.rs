use std::env;
use std::net::TcpListener;

use my_app::serve;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenv::from_filename(environment_file).ok();
    let database_host = dotenv::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind port 8888");
    serve(listener, &database_host, &database_name)?.await
}
