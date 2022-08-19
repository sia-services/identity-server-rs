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
pub fn ssl(http: &SSLConfig) -> SslAcceptorBuilder {
    let ssl = &http.ssl;

    let keypath = Path::new(&ssl.path);
    let keyfilepath = keypath.join(&ssl.keyfile);
    let certfilepath = keypath.join(&ssl.certfile);

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file(keyfilepath, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(certfilepath).unwrap();
    builder
}

use postgres_openssl::MakeTlsConnector;

pub fn create_db_pool(pg: deadpool_postgres::Config) -> Pool {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE)?;
    let connector = MakeTlsConnector::new(builder.build());

    let connector = native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let connector = MakeTlsConnector::new(connector);

    pg.create_pool(None, connector).unwrap()
}
