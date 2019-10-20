pub mod http_validate;

use {
  acme_lib::{
    self,
    persist::{FilePersist, Persist},
    {Account, Certificate, Directory, DirectoryUrl},
  },
  http_validate::HttpValidate,
  std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
  },
};

pub struct AcmeConfig {
  pub account: String,
  pub domain: String,
  pub use_staging: bool,
  pub persist: PersistConfig,
}

pub enum PersistConfig {
  File { storage_path: PathBuf },
}

pub struct Acme {
  config: AcmeConfig,
  challenges: HttpValidate,
}

impl Acme {
  pub fn new(config: AcmeConfig) -> Self {
    // TODO: move this http logic out of here.
    let challenges = HttpValidate::new("127.0.0.1".to_owned(), 9002, config.domain.clone());
    Self { config, challenges }
  }

  /// Import private key into persistence.
  pub fn import<R: Read>(self, _priv: R) {
    unimplemented!()
  }

  pub fn tls(&self) -> Result<Tls> {
    // TODO: move this http logic out of here.
    self.challenges.start().unwrap();

    let persist = match self.config.persist {
      PersistConfig::File { ref storage_path } => {
        let storage_path = storage_path.join("persistence");
        log::debug!("using persistence path: {:?}", storage_path);
        fs::create_dir_all(&storage_path)?;
        FilePersist::new(storage_path)
      }
    };
    let dir_url = match self.config.use_staging {
      true => DirectoryUrl::LetsEncryptStaging,
      false => DirectoryUrl::LetsEncrypt,
    };
    log::info!("using ACME url: {:?}", dir_url);
    let dir = Directory::from_url(persist, dir_url)?;

    log::debug!("using account: {}", self.config.account);
    let acc = dir.account(&self.config.account)?;

    log::debug!("using domain: {}", self.config.domain);
    let cert = match acc.certificate(&self.config.domain)? {
      Some(cert) => {
        let days_left = cert.valid_days_left();
        log::info!("days left on cert: {}", days_left);
        match days_left {
          // TODO: enable renew.
          // days_left if days_left <= RENEW_AT_DAYS_LEFT => self.renew_cert(acc)?,
          _ => cert,
        }
      }
      None => self.create_cert(acc)?,
    };

    Ok(Tls {
      cert: cert.certificate().to_owned(),
      key: cert.private_key().to_owned(),
    })
  }

  pub fn tls_to_dir(&self, dst: PathBuf) -> Result<TlsPaths> {
    fs::create_dir_all(&dst)?;
    let tls = self.tls()?;
    let cert_path = dst.join("cert");
    let key_path = dst.join("key");
    let mut cert_file = File::create(&cert_path)?;
    let mut key_file = File::create(&key_path)?;
    cert_file.write_all(&tls.cert.into_bytes())?;
    cert_file.sync_all()?;
    key_file.write_all(&tls.key.into_bytes())?;
    key_file.sync_all()?;
    Ok(TlsPaths {
      cert: cert_path,
      key: key_path,
    })
  }

  // fn renew_cert(&self) -> Result<Certificate> {
  //   log::warn!("renewing ACME certificate");
  //   unimplemented!()
  // }

  fn create_cert<P: Persist>(&self, acc: Account<P>) -> Result<Certificate> {
    log::warn!("creating ACME certificate");

    // Order a new TLS certificate for a domain.
    let mut ord_new = acc.new_order(&self.config.domain, &[])?;

    let ord_csr = loop {
      if let Some(ord_csr) = ord_new.confirm_validations() {
        break ord_csr;
      }

      let auths = ord_new.authorizations()?;
      for auth in auths.into_iter() {
        let chall = auth.http_challenge();

        let token = chall.http_token();
        let proof = chall.http_proof();
        self.challenges.insert_challenge(token.to_owned(), proof)?;

        // TODO: validate ourselves that we can hit the HTTP proof, as a simple measure to
        // avoid hitting the ACME API when not viable.

        chall.validate(5000)?;
      }

      // Update the state against the ACME API.
      ord_new.refresh()?;
    };

    // Ownership is proven. Create a private/public key pair for
    // the certificate.
    let (pkey_pri, pkey_pub) = acme_lib::create_p384_key();

    // Submit the CSR. This causes the ACME provider to enter a
    // state of "processing" that must be polled until the
    // certificate is either issued or rejected. Again we poll
    // for the status change.
    let ord_cert = ord_csr.finalize_pkey(pkey_pri, pkey_pub, 5000)?;

    // Now download the certificate. Also stores the cert in
    // the persistence.
    let cert = ord_cert.download_and_save_cert()?;

    Ok(cert)
  }

  // TODO: implement a monitoring solution to periodically check if the cert needs renewal.
  // Currently it's possible that we'll simply run the normal TLS check at a very slow interval,
  // however i want to be sure what methods the ACME Protocol intends clients to check with
  // to verify ownership.
  pub fn monitor(self) {
    unimplemented!()
  }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  CertMissingAfterCreation,
  ChallengeLockPoisoned,
  AcmeLib(acme_lib::Error),
  IoError(std::io::Error),
  ServerConfigError(rocket::config::ConfigError),
}

impl From<acme_lib::Error> for Error {
  fn from(err: acme_lib::Error) -> Self {
    Self::AcmeLib(err)
  }
}
impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Self::IoError(err)
  }
}
impl From<rocket::config::ConfigError> for Error {
  fn from(err: rocket::config::ConfigError) -> Self {
    Self::ServerConfigError(err)
  }
}

pub struct TlsPaths {
  pub cert: PathBuf,
  pub key: PathBuf,
}

pub struct Tls {
  pub cert: String,
  pub key: String,
}
