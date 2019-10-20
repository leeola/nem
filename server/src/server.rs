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
  pub tls: TlsConfig,
}

/// TLS related configuration.
pub enum TlsConfig {
  /// No TLS configured.
  None,
  Manual {
    /// The path to a certificate chain corresponding to the private key.
    ///
    /// The certificate chain must be in X.509 PEM format.
    certs: String,
    /// The path to a private key file corresponding to the certificate chain.
    ///
    /// The private key must be an RSA key in either PKCS#1 or PKCS#8 PEM format.
    key: String,
  },
  Automatic(TlsAutomaticConfig),
}

pub struct TlsAutomaticConfig {
  /// The Acme account to use, typically a LetsEncrypt account email.
  pub account: String,
  /// The Acme domain that this server will be running on.
  pub domain: String,
  /// IMPORTANT: Use the staging environment when experimenting with LetsEncrypt to avoid
  /// Rate Limits.
  ///
  /// Not using staging during testing is likely to cause your account to be blocked from
  /// LetsEncrypt.
  pub use_staging: bool,
  /// The port to run the ACME HTTP challenge server on.
  ///
  /// Typically 443 if running in a normal environment.
  pub port: u16,
  /// The address to run the ACME HTTP challenge server on.
  ///
  /// Typically you'll want to set this to 0.0.0.0 if you want it reachable outside of your
  /// local machine.
  pub address: String,
}

pub struct Server {
  pub config: Config,
  pub server: Rocket,
}

impl Server {
  pub fn new(config: Config) -> Result<Server, InitError> {
    let rocket_config = new_rocket_config_builder(&config);

    // apply TLS, if configured.
    let rocket_config = match config.tls.as_ref() {
      TlsConfig::None => rocket_config,
      TlsConfig::Manual { certs, key } => rocket_config.tls(certs, key),
      TlsConfig::Automatic(_) => return Err(InitError::InvalidServerTlsVariant),
    };

    Ok(Self {
      config: config,
      server: main_rocket_from_config(rocket_config)?,
    })
  }

  pub fn listen(self) {
    self.server.launch();
  }
}

pub struct ServerWithAcme {}

impl ServerWithAcme {
  pub fn new_and_listen(config: Config) -> Result<(), InitError> {
    let rocket_config = new_rocket_config_builder(&config);
    let automatic_tls_config = match config.tls {
      TlsConfig::Automatic(c) => c,
      TlsConfig::None | TlsConfig::Manual { .. } => return Err(InitError::InvalidServerTlsVariant),
    };

    // first, create Acme and fetch TLS from it. This will create everything needed,
    // or fetch existing TLS if not needed.
    let acme = Acme::new(AcmeConfig {
      account: automatic_tls_config.account,
      domain: automatic_tls_config.domain,
      use_staging: automatic_tls_config.use_staging,
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
    let rocket_config = rocket_config.tls(certs, key);

    // TODO: monitor TLS for invalidation.
    // acme.monitor();

    let rocket = main_rocket_from_config(rocket_config)?;
    rocket.launch();
    unreachable!()
  }
}

fn new_rocket_config_builder(config: &Config) -> rocket::config::ConfigBuilder {
  // matching default rocket behavior, since nem_server doesn't expose this as configuration, yet.
  #[cfg(debug_assertions)]
  let rocket_env = rocket::config::Environment::Development;
  #[cfg(not(debug_assertions))]
  let rocket_env = rocket::config::Environment::Production;

  rocket::config::Config::build(rocket_env)
    .address(&config.address)
    .port(config.port)
}

fn main_rocket_from_config(
  rocket_config: rocket::config::ConfigBuilder,
) -> Result<Rocket, InitError> {
  let rocket_config = rocket_config.finalize()?;
  let rocket_server = rocket::custom(rocket_config)
    .mount("/public", StaticFiles::from("./public"))
    .manage(Template::new("./templates").expect("templates failed to initialize"))
    .mount("/", routes![handlers::index]);
  Ok(rocket_server)
}

// fn acme_setup(rocket_config: rocket::config::ConfigBuilder) ->

impl From<rocket::config::ConfigError> for InitError {
  fn from(err: rocket::config::ConfigError) -> Self {
    InitError::RocketConfig(err)
  }
}

impl TlsConfig {
  pub fn as_ref(&self) -> &Self {
    self
  }
  pub fn is_automatic(&self) -> bool {
    match self {
      TlsConfig::None | TlsConfig::Manual { .. } => false,
      TlsConfig::Automatic { .. } => true,
    }
  }
  pub fn is_using_tls(&self) -> bool {
    match self {
      TlsConfig::None => false,
      TlsConfig::Manual { .. } | TlsConfig::Automatic { .. } => true,
    }
  }
}
