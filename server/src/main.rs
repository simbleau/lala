#[macro_use]
extern crate rocket;
use ringbuffer::RingBufferExt;
use rocket::State;
use rocket_client_addr::ClientAddr;
use std::sync::Mutex;
mod state;
use state::Config;

#[catch(404)]
fn not_found() -> &'static str {
    "Resource not found."
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Internal error occurred."
}

#[get("/status")]
fn status(state: &State<Mutex<Config>>) -> String {
    let config = state.lock().unwrap();
    format!("{}", config.status())
}

#[post("/signal")]
fn signal(state: &State<Mutex<Config>>, client_addr: &ClientAddr) -> String {
    let mut config = state.lock().unwrap();
    config.signal(client_addr);
    format!("{}", config.status())
}

#[post("/silence")]
fn silence(state: &State<Mutex<Config>>, client_addr: &ClientAddr) -> String {
    let mut config = state.lock().unwrap();
    config.silence(client_addr);
    format!("{}", config.status())
}

#[get("/history")]
fn history(state: &State<Mutex<Config>>) -> String {
    let config = state.lock().unwrap();
    let history = config.get_history();
    let mut values = Vec::new();
    for entry in history.iter().rev() {
        values.push(serde_json::to_value(entry).unwrap());
    }
    serde_json::value::Value::Array(values).to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(Config::default()))
        .mount("/", routes![status, signal, silence, history])
        .register("/", catchers![not_found, internal_error])
}
