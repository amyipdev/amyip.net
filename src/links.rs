use rocket::response::Redirect;
use rocket::http::Status;

#[get("/l/<link>")]
pub fn link(link: String) -> Result<Redirect, Status> {
    Ok(Redirect::to(match link.as_str() {
        "gh" => "https://github.com/amyipdev",
        "csufdivest" => "mailto:presidentalva@fullerton.edu?cc=dforgues@fullerton.edu&subject=CSUF%20Must%20Divest%20and%20Call%20for%20a%20Ceasefire%20Now!&body=Hello%2C%0A%0AI%20write%20to%20the%20CSUF%20administration%2C%20and%20to%20President%20Alva%20specifically%2C%20demanding%20that%20the%20University%20(and%20its%20auxiliaries%2C%20as%20well%20as%20the%20Philanthropic%20Foundation)%20immediately%20divest%20from%20companies%20that%20financially%20and%20materially%20support%20the%20genocide%20in%20Gaza.%20%0A%0AAll%20universities%20in%20Gaza%20have%20been%20flattened%20to%20the%20ground.%20Tens%20of%20thousands%20have%20been%20slaughtered.%20Millions%20have%20been%20displaced.%20Palestine%20has%20been%20occupied%20for%20nearly%2076%20years%2C%20and%20this%20administration%20is%20directly%20complicit%20in%20it.%0A%0AYou%20are%20using%20our%20tuition%20money%20to%20fund%20genocide.%20This%20ends%20now.%20No%20money%20for%20genocide%2C%20no%20money%20for%20apartheid%2C%20and%20no%20money%20for%20occupation.%20CSUF%2C%20divest%20from%20death!",
        _ => return Err(Status::NotFound),
    }))
}
