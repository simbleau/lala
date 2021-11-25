#[macro_use]
extern crate rocket;
use rocket::{http::Status, State};
use rocket_client_addr::ClientAddr;
use std::sync::Mutex;
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(ServerState::default()))
        .mount("/", routes![get_servers, put_server])
        .register("/", catchers![not_found, internal_error])
}
