#![feature(proc_macro_hygiene, decl_macro)]

use {
  clap::{App, Arg, ArgGroup, SubCommand},
  mnemosyne_server::server::{Config, Server, ServerWithAcme, TlsAutomaticConfig, TlsConfig},
  std::path::PathBuf,
};

fn main() {
  env_logger::init_from_env(env_logger::Env::new().filter_or("NEM_LOG", "info"));

  let matches = App::new("nem-server")
    .about("Does awesome things")
    .subcommand(
      SubCommand::with_name("serve")
        .about("Run the Nem server")
        .arg(
          Arg::with_name("storage")
            .long("storage")
            .value_name("DIR")
            .help("A directory where Nem server can store files, databses, etc")
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("address")
            .long("address")
            .value_name("ADDR")
            .default_value("127.0.0.1")
            .help("A local address to run the Nem server on. Typically 127.0.0.1 or 0.0.0.0")
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("port")
            .long("port")
            .value_name("PORT")
            .default_value("9001")
            .help("A port to run the Nem server on")
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("https-cert")
            .long("https-cert")
            .value_name("FILE")
            .help("A TLS certificate chain, for manually enabling HTTPS")
            .takes_value(true),
        )
        .arg(
          Arg::with_name("https-key")
            .long("https-key")
            .value_name("FILE")
            .help("A TLS private key, for manually enabling HTTPS")
            .takes_value(true),
        )
        .arg(
          Arg::with_name("acme-account")
            .long("acme-account")
            .value_name("ACCOUNT")
            .help("An ACME account, typically an email for LetsEncrypt")
            .takes_value(true),
        )
        // TODO: support multiple domains in the future.
        .arg(
          Arg::with_name("acme-domain")
            .long("acme-domain")
            .value_name("DOMAIN")
            .help("An ACME domain to manage")
            .takes_value(true),
        )
        .arg(
          Arg::with_name("acme-http-port")
            .long("acme-http-port")
            .value_name("PORT")
            .help(
              "The port that ACME HTTP challenges will run on.\
               Typically 80 [default: 9002]",
            )
            // Disabling default inside group, see: https://github.com/clap-rs/clap/issues/1586
            //.default_value("9002")
            .takes_value(true),
        )
        .arg(
          Arg::with_name("acme-http-address")
            .long("acme-http-address")
            .value_name("PORT")
            .help(
              "The address that ACME HTTP challenges will run on.\
               Typically 127.0.0.1 or 0.0.0.0 [default: 127.0.0.1]",
            )
            // Disabling default inside group, see: https://github.com/clap-rs/clap/issues/1586
            //.default_value("0.0.0.0")
            .takes_value(true),
        )
        .arg(
          Arg::with_name("lets-encrypt-staging")
            .long("lets-encrypt-staging")
            .help(
              "Use the staging server for LetsEncrypt, good for development testing and\
               avoiding Rate Limit blocking.",
            )
            .takes_value(false),
        )
        .group(
          ArgGroup::with_name("https")
            .multiple(true)
            .args(&["https-cert", "https-key"])
            .requires_all(&["https-cert", "https-key"])
            .conflicts_with_all(&["port", "acme"]),
        )
        .group(
          ArgGroup::with_name("acme")
            .multiple(true)
            .args(&[
              "acme-account",
              "acme-domain",
              "lets-encrypt-staging",
              "acme-http-port",
              "acme-http-address",
            ])
            .requires_all(&[
              "acme-account",
              "acme-domain",
              "acme-http-address",
              "acme-http-port",
            ])
            .conflicts_with_all(&["port", "https"]),
        ),
    )
    .get_matches();

  let matches = matches
    .subcommand_matches("serve")
    .expect("serve subcommand is required currently");

  // If it's ever desired to run LetsEncrypt production on Debug build we'll likely
  // add `--lets-encrypt-production` or something.
  let use_staging = match (
    cfg!(debug_assertions),
    matches.occurrences_of("lets-encrypt-staging"),
  ) {
    (true, 0) => {
      log::warn!("using LetsEncrypt Staging because nem-server is built with debug, and --lets-encrypt-staging is not specified");
      true
    }
    _ => matches.is_present("lets-encrypt-staging"),
  };

  let tls = match (
    matches.is_present("https-cert"),
    matches.is_present("acme-account"),
  ) {
    (false, false) => TlsConfig::None,
    (true, false) => TlsConfig::Manual {
      certs: matches
        .value_of("https-cert")
        .map(|s| s.to_owned())
        .expect("--https-cert impossibly missing"),
      key: matches
        .value_of("https-key")
        .map(|s| s.to_owned())
        .expect("--https-key impossibly missing"),
    },
    (false, true) => TlsConfig::Automatic(TlsAutomaticConfig {
      account: matches
        .value_of("acme-account")
        .map(|s| s.to_owned())
        .expect("--acme-account impossibly missing"),
      domain: matches
        .value_of("acme-domain")
        .map(|s| s.to_owned())
        .expect("--acme-domain impossibly missing"),
      use_staging,
      port: matches
        .value_of("acme-http-port")
        // Manually specifying the default, due to:
        // https://github.com/clap-rs/clap/issues/1586
        .map_or_else(
          || 9002,
          |s| s.parse::<u16>().expect("invalid --acme-http-port"),
        ),
      //.expect("--acme-http-port impossibly missing"),
      address: matches
        .value_of("acme-http-address")
        // Manually specifying the default, due to:
        // https://github.com/clap-rs/clap/issues/1586
        .map_or_else(|| "".to_owned(), |s| s.to_owned()),
      //.expect("--acme-http-address impossibly missing"),
    }),
    _ => unreachable!("CLI parser should have prevented both manual and automatic flags"),
  };
  let storage = matches
    .value_of("storage")
    .map(|s| PathBuf::from(s))
    .expect("missing --storage");
  let address = matches
    .value_of("address")
    .map(|s| s.to_owned())
    .expect("missing --address");
  let port = match tls.is_using_tls() {
    true => 443,
    false => matches
      .value_of("port")
      .map(|s| s.parse::<u16>().expect("invalid --port"))
      .expect("missing --port"),
  };

  let server_config = Config {
    storage,
    port,
    address,
    tls,
  };

  if !server_config.tls.is_automatic() {
    Server::new(server_config)
      .expect("failed to build server")
      .listen();
  } else {
    ServerWithAcme::new_and_listen(server_config).expect("failed to build server");
  }
}
