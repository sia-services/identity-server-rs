mod database;
mod errors;
mod handlers;
mod setup;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=debug");
    // std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();
    env_logger::init_from_env(::env_logger::Env::default().default_filter_or("info"));
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: setup::IdentityServerConfig = config_.try_deserialize().unwrap();
    let ssl_builder = setup::ssl(&config.ssl);

    let pool = setup::create_db_pool(config.pg);

    log::info!("Server running at http://{}/", config.server_addr);

    let server = HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(logger)
            .service(handlers::hello)
    })
    .bind_openssl(config.server_addr.clone(), ssl_builder)?
    .run();

    server.await
}
