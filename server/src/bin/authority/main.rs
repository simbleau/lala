#[macro_use]
extern crate rocket;
use rocket::{http::Status, response::Redirect, Config, State};
use rocket_client_addr::ClientAddr;
use server_util::AUTHORITY_PORT;
use std::{net::Ipv4Addr, sync::Mutex};
mod server_state;
use server_state::ServerState;

#[catch(404)]
fn not_found() -> &'static str {
    "Resource not found."
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Internal error occurred."
}

#[get("/servers")]
fn get_servers(state: &State<Mutex<ServerState>>) -> String {
    let state = state.lock().unwrap();
    serde_json::to_string(&state.alarms()).unwrap()
}

#[put("/servers")]
fn put_server(
    state: &State<Mutex<ServerState>>,
    client_addr: &ClientAddr,
) -> Status {
    let mut state = state.lock().unwrap();
    state.checkin(client_addr);
    state.prune();
    Status::Created
}

#[post("/status?<server>")]
fn status_redirect(server: &str) -> Redirect {
    Redirect::to(format!("{}/status", server))
}

#[post("/signal?<server>")]
fn signal_redirect(server: &str) -> Redirect {
    Redirect::to(format!("{}/signal", server))
}

#[post("/silence?<server>")]
fn silence_redirect(server: &str) -> Redirect {
    Redirect::to(format!("{}/silence", server))
}

#[get("/history?<server>")]
fn history_redirect(server: &str) -> Redirect {
    Redirect::to(format!("{}/history", server))
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        port: AUTHORITY_PORT,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::default()
    };

    rocket::custom(&config)
        .manage(Mutex::new(ServerState::default()))
        .mount("/", routes![get_servers, put_server])
        .mount(
            "/",
            routes![
                status_redirect,
                signal_redirect,
                silence_redirect,
                history_redirect
            ],
        )
        .register("/", catchers![not_found, internal_error])
}
