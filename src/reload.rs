use std::convert::Infallible;

use hmac::Mac;
use rocket::http::Status;
use rocket::request::{self, FromRequest};

struct Valid(String);

#[cfg(feature = "reload_github")]
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Valid {
    type Error = Infallible;
    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("X-GitHub-Event") {
            Some("push") | Some("ping") => {}
            _ => return request::Outcome::Forward(Status::Unauthorized),
        };
        match request.headers().get_one("X-Hub-Signature-256") {
            Some(tok) => request::Outcome::Success(Self(tok.to_string())),
            None => request::Outcome::Forward(Status::Unauthorized),
        }
    }
}

#[cfg(feature = "reload_github")]
#[post("/reload_github", data = "<input>")]
#[allow(private_interfaces)]
pub fn reload_github(t: Valid, input: &[u8]) -> Status {
    let mut bstr = "sha256=".to_string();
    let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(
        env!("GITHUB_WEBHOOK_AUTHENTICATION_AMYIPNET").as_bytes(),
    )
    .expect("failed to get secret key");
    mac.update(input);
    let r = mac.finalize();
    bstr.push_str(&hex::encode(r.into_bytes()));
    if bstr != t.0 {
        return Status::Unauthorized;
    }
    if let Ok(fork::Fork::Child) = fork::daemon(false, false) {
        std::process::Command::new("bash")
            .arg("./reload-tmux.sh")
            .current_dir(rocket::fs::relative!("."))
            .spawn()
            .expect("failed to spawn reloader");
    }
    Status::Ok
}

#[cfg(not(feature = "reload_github"))]
#[post("/reload_github")]
pub fn reload_github() -> rocket::response::status::NotFound<()> {
    println!("test");
    rocket::response::status::NotFound(())
}
