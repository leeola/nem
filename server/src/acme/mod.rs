use {
  acme_lib::{
    self,
    persist::FilePersist,
    {Certificate, Directory, DirectoryUrl},
  },
  std::{
    fs::File,
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
}

impl Acme {
  pub fn new(config: AcmeConfig) -> Self {
    Self { config }
  }

  /// Import private key into persistence.
  pub fn import<R: Read>(self, _priv: R) {
    unimplemented!()
  }

  pub fn tls(&self) -> Result<Tls> {
    let persist = match self.config.persist {
      PersistConfig::File { ref storage_path } => {
        FilePersist::new(storage_path.join("persistence"))
      }
    };
    let dir_url = match self.config.use_staging {
      true => DirectoryUrl::LetsEncryptStaging,
      false => DirectoryUrl::LetsEncrypt,
    };
    let dir = Directory::from_url(persist, dir_url)?;
    let acc = dir.account("foo@bar.com")?;

    let cert = match acc.certificate(&self.config.domain)? {
      Some(cert) => {
        let days_left = cert.valid_days_left();
        log::info!("days left on cert: {}", days_left);
        match days_left {
          // TODO: enable renew.
          // days_left if days_left <= RENEW_AT_DAYS_LEFT => self.renew_cert()?,
          _ => cert,
        }
      }
      None => self.create_cert()?,
    };

    Ok(Tls {
      cert: cert.certificate().to_owned(),
      key: cert.private_key().to_owned(),
    })
  }

  pub fn tls_to_dir(&self, dst: PathBuf) -> Result<TlsPaths> {
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

  fn create_cert(&self) -> Result<Certificate> {
    log::warn!("creating ACME certificate");
    unimplemented!()
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
  AcmeLib(acme_lib::Error),
  IoError(std::io::Error),
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

pub struct TlsPaths {
  pub cert: PathBuf,
  pub key: PathBuf,
}

pub struct Tls {
  pub cert: String,
  pub key: String,
}
