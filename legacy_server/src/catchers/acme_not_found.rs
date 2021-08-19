use {
    crate::acme::Domain,
    rocket::{catch, response::Redirect, Request, State},
};

#[catch(404)]
pub fn acme_not_found(req: &Request) -> Redirect {
    let url = format!(
        "https://{}{}",
        req.guard::<State<Domain>>().unwrap().0,
        req.uri()
    );
    log::info!("redirecting http to: {}", url);
    Redirect::permanent(url)
}
