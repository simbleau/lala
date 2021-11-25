#[macro_use]
extern crate rocket;
use ringbuffer::RingBufferExt;
use rocket::{Config, State};
use rocket_client_addr::ClientAddr;
use server_util::{ALARM_PORT, CORS};
use std::{net::Ipv4Addr, sync::Mutex};
mod alarm_state;
use alarm_state::AlarmState;

#[catch(404)]
fn not_found() -> &'static str {
    "Resource not found."
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Internal error occurred."
}

#[get("/status")]
fn status(state: &State<Mutex<AlarmState>>) -> String {
    let state = state.lock().unwrap();
    serde_json::to_string(state.status()).unwrap()
}

#[post("/signal")]
fn signal(
    state: &State<Mutex<AlarmState>>,
    client_addr: &ClientAddr,
) -> String {
    let mut state = state.lock().unwrap();
    state.signal(client_addr);
    serde_json::to_string(state.status()).unwrap()
}

#[post("/silence")]
fn silence(
    state: &State<Mutex<AlarmState>>,
    client_addr: &ClientAddr,
) -> String {
    let mut state = state.lock().unwrap();
    state.silence(client_addr);
    serde_json::to_string(state.status()).unwrap()
}

#[get("/history")]
fn history(state: &State<Mutex<AlarmState>>) -> String {
    let state = state.lock().unwrap();
    let mut values = Vec::new();
    for entry in state.history().entries().iter().rev() {
        values.push(serde_json::to_value(entry).unwrap());
    }
    serde_json::value::Value::Array(values).to_string()
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        port: ALARM_PORT,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::default()
    };

    rocket::custom(&config)
        .attach(CORS)
        .manage(Mutex::new(AlarmState::default()))
        .mount("/", routes![status, signal, silence, history])
        .register("/", catchers![not_found, internal_error])
}
