#[macro_use]
extern crate rocket;
use ringbuffer::RingBufferExt;
use rocket::{http::Status, Config, State};
use rocket_client_addr::ClientAddr;
use server_util::{models::AlarmStatus, PreflightCORS, ALARM_PORT, CORS};
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
    serde_json::to_string(&state.status()).unwrap()
}

#[options("/signal")]
fn signal_preflight() -> PreflightCORS {
    CORS
}

#[post("/signal")]
fn signal(
    state: &State<Mutex<AlarmState>>,
    client_addr: &ClientAddr,
) -> Status {
    let mut state = state.lock().unwrap();
    if matches!(state.status(), AlarmStatus::Off) {
        state.signal(client_addr);
    }
    Status::Ok
}

#[options("/silence")]
fn silence_preflight() -> PreflightCORS {
    CORS
}

#[post("/silence")]
fn silence(
    state: &State<Mutex<AlarmState>>,
    client_addr: &ClientAddr,
) -> Status {
    let mut state = state.lock().unwrap();
    if matches!(state.status(), AlarmStatus::On) {
        state.silence(client_addr);
    }
    Status::Ok
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
        .mount("/", routes![signal_preflight, silence_preflight])
        .register("/", catchers![not_found, internal_error])
}
