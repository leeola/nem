use {
  crate::{error::InitError, handlers, states::template::Template},
  rocket::{routes, Rocket},
  rocket_contrib::serve::StaticFiles,
};

/// The base configuration for the Nem server.
pub struct Config {
  /// The port to run Nem Server on.
  pub port: u16,
  /// The address to bind Nem to.
  ///
  /// Typically you'll want to bind this to 0.0.0.0 if you want it reachable outside of your
  /// local machine.
  pub address: String,
  /// Optional, TLS related configuration.
  ///
  /// If set, TLS is used.
  pub tls: Option<TLSConfig>,
}

/// TLS related configuration.
pub struct TLSConfig {
  /// The path to a certificate chain corresponding to the private key.
  ///
  /// The certificate chain must be in X.509 PEM format.
  pub certs: String,
  /// The path to a private key file corresponding to the certificate chain.
  ///
  /// The private key must be an RSA key in either PKCS#1 or PKCS#8 PEM format.
  pub key: String,
}

pub fn build(config: Config) -> Result<Rocket, InitError> {
  // matching default rocket behavior, since nem_server doesn't expose this as configuration, yet.
  #[cfg(debug_assertions)]
  let rocket_env = rocket::config::Environment::Development;
  #[cfg(not(debug_assertions))]
  let rocket_env = rocket::config::Environment::Production;

  let rocket_config = rocket::config::Config::build(rocket_env)
    .address(config.address)
    .port(config.port);

  // apply TLS, if configured.
  let rocket_config = match config.tls {
    Some(tls_config) => rocket_config.tls(tls_config.certs, tls_config.key),
    None => rocket_config,
  };

  let rocket_config = rocket_config.finalize()?;
  let server = rocket::custom(rocket_config)
    .mount("/public", StaticFiles::from("./public"))
    .manage(Template::new("./templates").expect("templates failed to initialize"))
    .mount("/", routes![handlers::index]);
  Ok(server)
}

impl From<rocket::config::ConfigError> for InitError {
  fn from(err: rocket::config::ConfigError) -> Self {
    InitError::RocketConfig(err)
  }
}
