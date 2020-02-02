// TODO: Attempt to make configuration defaults from this file used for the defaults in various
// configuration interfaces, like CLI. Eg, rather than have CLI and the GUI both hardcode defaults,
// they should depend on the default state of these configs.

/// The base configuration for the Nem server.
pub struct Config {
    /// A directory where data required to persist between server restarts can be stored.
    ///
    /// What data is stored depends on the configuration.
    pub storage: PathBuf,
    /// Nem configuration per environment type.
    ///
    /// Nem requires different behaviors depending on the intended home-user-centric Environments.
    /// See each environment variant for documentation.
    pub environment: Environment,
}

pub enum Environment {
    Dev(DevConfig),
    Home,
    Public,
}

pub struct DevConfig {
    /// The port to run Nem Server on.
    ///
    /// Defaults to 9000.
    pub port: u16,
    /// The address to bind Nem to.
    ///
    /// Defaults to 127.0.0.1.
    pub address: String,
    /// TLS options for the development focused configuration.
    pub tls: DevTls,
}

/// TLS related configuration.
pub enum DevTls {
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
    /// Acme managed TLS for Home servers, requires DNS challenges.
    ///
    /// NOT SUPPORTED YET.
    AcmeHome,
    AcmePublic(AcmeConfig),
}

pub struct AcmePublicConfig {
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
    /// The type of ACME challenge to use.
    pub challenge: AcmePublicChallenge,
}

pub enum AcmePublicChallenge {
    AutoHttp(AcmeHttpChallengeConfig),
    /// Configuration for DNS provider APIs to pass DNS challenges.
    ///
    /// NOT SUPPORTED YET.
    AutoDns,
}

pub struct AcmeHttpChallengeConfig {
    /// The port to run the ACME HTTP challenge server on.
    ///
    /// Typically 80 if running in a normal environment.
    port: u16,
    /// The address to run the ACME HTTP challenge server on.
    ///
    /// Typically you'll want to set this to 0.0.0.0 if you want it reachable outside of your
    /// local machine.
    address: String,
}
