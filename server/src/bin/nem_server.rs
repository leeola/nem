#![feature(proc_macro_hygiene, decl_macro)]

use {
  nem_server::server,
  std::convert::{TryFrom, TryInto},
  std::path::PathBuf,
  structopt::StructOpt,
};

/// The Nem Server.
#[derive(StructOpt)]
pub struct CLIConfig {
  #[structopt(subcommand)]
  pub cmd: CLICommand,
}

#[derive(StructOpt)]
pub enum CLICommand {
  Serve(ServeCmdConfig),
}

#[derive(StructOpt)]
pub struct ServeCmdConfig {
  /// The port to run the Nem Server on.
  #[structopt(default_value = "4300")]
  pub port: u16,
  /// The address to run the Nem Server on.
  ///
  /// Typically you'll want to set this to 0.0.0.0 if you want it reachable outside of your
  /// local machine.
  #[structopt(default_value = "127.0.0.1")]
  pub address: String,
  /// The path to a certificate chain corresponding to the private key.
  ///
  /// The certificate chain must be in X.509 PEM format.
  ///
  /// If you're running this locally, you can ignore this field. This is used to enable HTTPS
  /// when running on servers with a domain.
  pub certs: Option<PathBuf>,
  /// The path to a private key file corresponding to the certificate chain.
  ///
  /// The private key must be an RSA key in either PKCS#1 or PKCS#8 PEM format.
  ///
  /// If you're running this locally, you can ignore this field. This is used to enable HTTPS
  /// when running on servers with a domain.
  pub key: Option<PathBuf>,
}

fn main() {
  let cli_config = CLIConfig::from_args();
  let server_config = cli_config.try_into().expect("invalid CLI Config");
  server::build(server_config)
    .expect("failed to build server")
    .launch();
}

impl TryFrom<CLIConfig> for server::Config {
  type Error = &'static str;
  fn try_from(cli_config: CLIConfig) -> std::result::Result<Self, Self::Error> {
    let server_config = match cli_config.cmd {
      CLICommand::Serve(serve_config) => {
        let certs = serve_config
          .certs
          .map(|cert| {
            let cert = cert
              .to_str()
              .ok_or_else(|| "--certs must be valid UTF8 String")?
              .to_owned();
            Ok(Some(cert))
          })
          .unwrap_or_else(|| Ok(None))?;
        let key = serve_config
          .key
          .map(|key| {
            let key = key
              .to_str()
              .ok_or_else(|| "--key must be valid UTF8 String")?
              .to_owned();
            Ok(Some(key))
          })
          .unwrap_or_else(|| Ok(None))?;
        Self {
          port: serve_config.port,
          address: serve_config.address,
          tls: match (certs, key) {
            (Some(certs), Some(key)) => Some(server::TLSConfig { certs, key }),
            (None, None) => None,
            _ => return Err("--certs and --key must be passed together or not at all"),
          },
        }
      }
    };
    Ok(server_config)
  }
}
