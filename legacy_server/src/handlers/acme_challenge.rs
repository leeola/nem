use {
    crate::{acme::MaybeChallenges, Error, Result},
    rocket::{get, http::Status, Response, State},
    std::io::Cursor,
};

#[get("/.well-known/acme-challenge/<req_token>")]
pub fn acme_challenge(
    req_token: String,
    maybe_challenges: State<MaybeChallenges>,
) -> Result<Response<'_>> {
    log::info!("challenge request for token: {}", req_token);

    let mut maybe_challenges = maybe_challenges
        .lock()
        .map_err(|_| Error::Internal("Poisoned ACME Challenge Lock".to_owned()))?;

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
