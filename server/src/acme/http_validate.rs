use {
  super::{Error, Result},
  rocket::{
    catch, catchers, get, http::Status, response::Redirect, routes, Request, Response, State,
  },
  std::{
    collections::HashMap,
    io::Cursor,
    sync::{Arc, Mutex},
    thread,
  },
};

type MaybeChallenges = Arc<Mutex<Option<HashMap<String, String>>>>;

struct Domain(String);

#[get("/.well-known/acme-challenge/<req_token>")]
fn validate(
  req_token: String,
  maybe_challenges: State<MaybeChallenges>,
) -> crate::error::Result<Response<'_>> {
  log::info!("challenge request for token: {}", req_token);

  let mut maybe_challenges = maybe_challenges
    .lock()
    .map_err(|_| crate::error::Error::Internal("Poisoned ACME Challenge Lock".to_owned()))?;

  let challs = match maybe_challenges.as_mut() {
    Some(challs) => challs,
    None => {
      log::warn!("token requested without Challenges");
      let mut res = Response::new();
      res.set_status(Status::NotFound);
      return Ok(res);
    }
  };

  let proof = match challs.get(&req_token) {
    Some(proof) => proof,
    None => {
      log::warn!("no Challenge found for requested token: {}", req_token);
      let mut res = Response::new();
      res.set_status(Status::NotFound);
      return Ok(res);
    }
  };

  let mut res = Response::new();
  res.set_sized_body(Cursor::new(proof.clone()));
  Ok(res)
}

#[catch(404)]
fn not_found(req: &Request) -> Redirect {
  let url = format!(
    "https://{}{}",
    req.guard::<State<Domain>>().unwrap().0,
    req.uri()
  );
  log::info!("redirecting http to: {}", url);
  Redirect::permanent(url)
}

pub struct HttpValidate {
  address: String,
  port: u16,
  domain: String,
  maybe_challenges: MaybeChallenges,
}

impl HttpValidate {
  pub fn new(address: String, port: u16, domain: String) -> Self {
    Self {
      address,
      port,
      domain,
      maybe_challenges: Arc::new(Mutex::new(None)),
    }
  }
  pub fn start(&self) -> Result<()> {
    let maybe_challenges = self.maybe_challenges.clone();
    let domain = Domain(self.domain.clone());

    #[cfg(debug_assertions)]
    let rocket_env = rocket::config::Environment::Development;
    #[cfg(not(debug_assertions))]
    let rocket_env = rocket::config::Environment::Production;
    let rocket_config = rocket::config::Config::build(rocket_env)
      .address(&self.address)
      .port(self.port)
      .workers(1)
      .finalize()?;

    thread::spawn(move || {
      log::info!("starting HTTP validation service on port 80");
      rocket::custom(rocket_config)
        .manage(maybe_challenges)
        .manage(domain)
        .mount("/", routes![validate])
        .register(catchers![not_found])
        .launch();
    });

    Ok(())
  }
  pub fn insert_challenge(&self, token: String, proof: String) -> Result<()> {
    let mut challs = self
      .maybe_challenges
      .lock()
      .map_err(|_| Error::ChallengeLockPoisoned)?;

    let challs = match challs.as_mut() {
      Some(challs) => challs,
      None => {
        *challs = Some(HashMap::new());
        challs.as_mut().expect("value impossibly missing")
      }
    };

    challs.insert(token, proof);

    Ok(())
  }
  pub fn clear_challenges(&self) -> Result<()> {
    let mut challs = self
      .maybe_challenges
      .lock()
      .map_err(|_| Error::ChallengeLockPoisoned)?;

    *challs = None;
    Ok(())
  }
}
