use rocket::response::Redirect;
use rocket::http::Status;

#[get("/l/<link>")]
pub fn link(link: String) -> Result<Redirect, Status> {
    Ok(Redirect::to(match link.as_str() {
        "gh" => "https://github.com/amyipdev",
        _ => return Err(Status::NotFound),
    }))
}
