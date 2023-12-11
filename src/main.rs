#[macro_use]
extern crate rocket;

mod ping;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                ping::ping,
                ping::ping_txt,
                ping::ping_uname,
                ping::ping_json
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
}

struct HighPerformanceCounter {}

#[rocket::async_trait]
impl rocket::fairing::Fairing for HighPerformanceCounter {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "COOP/COEP",
            kind: rocket::fairing::Kind::Response
        }
    }
    async fn on_response<'r>(&self, _request: &'r rocket::Request<'_>, response: &mut rocket::Response<'r>) {
        // If this isn't a 200 OK, we don't need COOP/COEP
        if response.status() != rocket::http::Status::Ok {
            return;
        }
        response.set_header(rocket::http::Header::new("Cross-Origin-Opener-Policy", "same-origin"));
        response.set_header(rocket::http::Header::new("Cross-Origin-Embedder-Policy", "require-corp"));
    }
}