mod database;
mod domain;
mod dto;
mod errors;
mod handlers;
mod identity;
mod setup;

use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;

// TODO: need use variables in testing/test-remote.http

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=debug");
    // std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();
    env_logger::init_from_env(::env_logger::Env::default().default_filter_or("info"));
    dotenv().ok();

    let config_ = ::config::Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: setup::ServerConfig = config_.try_deserialize().unwrap();
    let ssl_builder = setup::ssl(&config.ssl);

    let pool = setup::create_db_pool(config.pg);
    let identity_service = identity::Identity::new();
    let auth_token_middleware_factory = identity::AuthTokenMiddlewareFactory::new();

    log::info!("Server running at http://{}/", config.server_addr);

    let server = HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(identity_service.clone()))
            .wrap(logger)
            .wrap(auth_token_middleware_factory.clone())
            .service(handlers::hello)
            .service(handlers::login)
            .service(handlers::logout)
            .service(handlers::auth_scope())
    })
    .bind_openssl(config.server_addr.clone(), ssl_builder)?
    .run();

    server.await
}
