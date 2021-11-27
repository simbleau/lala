use rocket::fairing::Fairing;
use rocket::fairing::{Info, Kind};
use rocket::http::Header;
use rocket::response::{self, Responder};
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _request: &'r Request<'_>,
        response: &mut Response<'r>,
    ) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Credentials",
            "true",
        ));
    }
}

pub type PreflightCORS = CORS;

impl<'r> Responder<'r, 'static> for PreflightCORS {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .raw_header("Access-Control-Allow-Origin", "*")
            .raw_header("Access-Control-Allow-Credentials", "true")
            .raw_header("Access-Control-Allow-Methods", "*")
            .raw_header(
                "Access-Control-Allow-Credentials",
                "POST, PUT, GET, OPTIONS",
            )
            .ok()
    }
}
