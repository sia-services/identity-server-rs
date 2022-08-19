use openssl::ssl::{
    SslAcceptor, SslAcceptorBuilder, SslConnector, SslFiletype, SslMethod, SslVerifyMode,
};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
pub struct IdentityServerConfig {
    pub server_addr: String,
    pub ssl: SSLConfig,
    pub pg: deadpool_postgres::Config,
}

#[derive(Debug, Default, Deserialize)]
pub struct SSLConfig {
    pub path: String,
    pub keyfile: String,
    pub certfile: String,
}

/// load ssl keys
// to create a self-signed temporary cert for testing:
// `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
pub fn ssl(config: &SSLConfig) -> SslAcceptorBuilder {
    let keypath = Path::new(&config.path);
    let keyfilepath = keypath.join(&config.keyfile);
    let certfilepath = keypath.join(&config.certfile);

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file(keyfilepath, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(certfilepath).unwrap();
    builder
}

use postgres_openssl::MakeTlsConnector;

pub fn create_db_pool(pg: deadpool_postgres::Config) -> deadpool_postgres::Pool {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());
    pg.create_pool(None, connector).unwrap()
}
