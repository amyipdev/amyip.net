#[macro_use]
extern crate rocket;

mod ping;
mod reload;
mod links;

use rocket::data::ToByteUnit;
use rocket::fs::relative;

#[get("/")]
fn index() -> rocket::response::content::RawHtml<Option<String>> {
    rocket::response::content::RawHtml(
        match std::fs::read_to_string(relative!("svelte/public/index.html")) {
            Ok(s) => Some(s),
            Err(_) => None,
        },
    )
}

// TODO: HTTPS and HTTP2 support
// TODO: execlp into a bash script upon receiving GH webhook
// TODO: evaluate rocket_contrib StaticFiles over FileServer
#[launch]
fn rocket() -> _ {
    let config = rocket::Config {
        port: 8000,
        address: "::".parse::<std::net::IpAddr>().unwrap(),
        limits: rocket::data::Limits::new().limit("bytes", 32.kibibytes()),
        ..rocket::Config::release_default()
    };
    rocket::build()
        .configure(config)
        .mount(
            "/",
            routes![
                index,
                ping::ping,
                ping::ping_txt,
                ping::ping_uname,
                ping::ping_json,
                reload::reload_github,
                links::link
            ],
        )
        .mount(
            "/",
            rocket::fs::FileServer::from(relative!("svelte/public")),
        )
        .mount(
            "/build",
            rocket::fs::FileServer::from(relative!("svelte/wasm/pkg")).rank(20),
        )
        .attach(HighPerformanceCounter {})
        .attach(TerryPratchett {})
}

struct HighPerformanceCounter {}

#[rocket::async_trait]
impl rocket::fairing::Fairing for HighPerformanceCounter {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "COOP/COEP",
            kind: rocket::fairing::Kind::Response,
        }
    }
    async fn on_response<'r>(
        &self,
        _request: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        // If this isn't a 200 OK, we don't need COOP/COEP
        if response.status() != rocket::http::Status::Ok {
            return;
        }
        response.set_header(rocket::http::Header::new(
            "Cross-Origin-Opener-Policy",
            "same-origin",
        ));
        response.set_header(rocket::http::Header::new(
            "Cross-Origin-Embedder-Policy",
            "require-corp",
        ));
    }
}

struct TerryPratchett {}

#[rocket::async_trait]
impl rocket::fairing::Fairing for TerryPratchett {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "X-Clacks-Overhead",
            kind: rocket::fairing::Kind::Response,
        }
    }
    async fn on_response<'r>(
        &self,
        _request: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        if response.status() != rocket::http::Status::Ok {
            return;
        }
        response.set_header(rocket::http::Header::new(
            "X-Clacks-Overhead",
            "GNU Terry Pratchett, Aaron Swartz, Aaron Bushnell, Dennis Ritchie",
        ));
    }
}
