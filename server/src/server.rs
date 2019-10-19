use {
  crate::{
    acme::{Acme, AcmeConfig, PersistConfig},
    error::InitError,
    handlers,
    states::template::Template,
  },
  rocket::{routes, Rocket},
  rocket_contrib::serve::StaticFiles,
  std::path::PathBuf,
};

/// The base configuration for the Nem server.
pub struct Config {
  /// A directory where all server data is stored; DB, keys, config, etc.
  pub storage: PathBuf,
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
  pub tls: Option<TlsConfig>,
}

/// TLS related configuration.
pub enum TlsConfig {
  ManualTls {
    /// The path to a certificate chain corresponding to the private key.
    ///
    /// The certificate chain must be in X.509 PEM format.
    certs: String,
    /// The path to a private key file corresponding to the certificate chain.
    ///
    /// The private key must be an RSA key in either PKCS#1 or PKCS#8 PEM format.
    key: String,
  },
  AutomaticTls {
    /// The Acme account to use, typically a LetsEncrypt account email.
    account: String,
    /// The Acme domain that this server will be running on.
    domain: String,
    /// IMPORTANT: Use the staging environment when experimenting with LetsEncrypt to avoid
    /// Rate Limits.
    ///
    /// Not using staging during testing is likely to cause your account to be blocked from
    /// LetsEncrypt.
    use_staging: bool,
  },
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
    Some(tls_config) => match tls_config {
      TlsConfig::ManualTls { certs, key } => rocket_config.tls(certs, key),
      TlsConfig::AutomaticTls {
        account,
        domain,
        use_staging,
      } => {
        // first, create Acme and fetch TLS from it. This will create everything needed,
        // or fetch existing TLS if not needed.
        let acme = Acme::new(AcmeConfig {
          account,
          domain,
          use_staging,
          persist: PersistConfig::File {
            storage_path: config.storage.join("acme"),
          },
        });
        let tls_paths = acme.tls_to_dir(config.storage.join("rocket_tls"))?;
        let certs = tls_paths
          .cert
          .to_str()
          .ok_or_else(|| InitError::InvalidAcmePath(tls_paths.cert.clone()))?
          .to_owned();
        let key = tls_paths
          .key
          .to_str()
          .ok_or_else(|| InitError::InvalidAcmePath(tls_paths.key.clone()))?
          .to_owned();
        // acme.monitor();
        rocket_config.tls(certs, key)
      }
    },
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
