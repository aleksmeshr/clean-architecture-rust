use std::env;
use std::net::TcpListener;

use animal_facts_api::serve;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenv::from_filename(environment_file).ok();

    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind port 8888");
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    serve(listener, &database_name)?.await
}
