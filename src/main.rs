mod tweet;

use actix_web::{App, HttpServer};
use env_logger;
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| App::new().service(tweet::get))
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
