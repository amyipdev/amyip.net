#[macro_use]
extern crate rocket;

mod ping;

use colored::Colorize;
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
    match std::process::Command::new("npx")
        .arg("rollup")
        .arg("-c")
        .current_dir(relative!("svelte"))
        .status()
    {
        Ok(c) => {
            if !c.success() {
                eprintln!("{}", "amyip.net: svelte build failed".red());
                eprintln!("{}", "amyip.net: try running `npx rollup -c' in the svelte directory".red());
                std::process::exit(-2);
            }
            println!("{}", "amyip.net: svelte built".bright_cyan());
        },
        Err(e) => {
            eprintln!("{} {}", "amyip.net: could not run svelte build: ".bright_red(), e);
            std::process::exit(-1);
        }
    }
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
