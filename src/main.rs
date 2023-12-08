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
}
