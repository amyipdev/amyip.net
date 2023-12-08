#[get("/ping")]
pub fn ping() -> String {
    ping_txt()
}

#[get("/ping/txt")]
pub fn ping_txt() -> String {
    format!("Hello from amyip.net v{}", env!("CARGO_PKG_VERSION"))
}

#[get("/ping/uname")]
pub fn ping_uname() -> Option<String> {
    match uname::uname() {
        Ok(un) => Some(format!(
            "{} {} {} {} {}",
            un.sysname, un.nodename, un.release, un.version, un.machine
        )),
        Err(_) => return None,
    }
}

#[derive(rocket::serde::Serialize)]
struct PingJsonInfo {
    site_name: &'static str,
    version: String,
    repo: String,
    msrv: String,
}

#[get("/ping/json")]
#[allow(private_interfaces)]
pub fn ping_json() -> rocket::serde::json::Json<PingJsonInfo> {
    rocket::serde::json::Json(PingJsonInfo {
        site_name: "amyip.net",
        version: env!("CARGO_PKG_VERSION").to_string(),
        repo: env!("CARGO_PKG_REPOSITORY").to_string(),
        msrv: env!("CARGO_PKG_RUST_VERSION").to_string(),
    })
}
