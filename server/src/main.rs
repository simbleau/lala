#[macro_use]
extern crate rocket;
use std::sync::Mutex;

use ringbuffer::RingBufferExt;
use rocket::State;
use rocket_client_addr::ClientAddr;
mod state;
use state::AlarmState;

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
    let alarm = state.lock().unwrap();
    format!("{}", alarm.status())
}

#[post("/signal")]
fn signal(
    state: &State<Mutex<AlarmState>>,
    client_addr: &ClientAddr,
) -> String {
    let mut alarm = state.lock().unwrap();
    alarm.signal(client_addr);
    format!("{}", alarm.status())
}

#[post("/silence")]
fn silence(
    state: &State<Mutex<AlarmState>>,
    client_addr: &ClientAddr,
) -> String {
    let mut alarm = state.lock().unwrap();
    alarm.silence(client_addr);
    format!("{}", alarm.status())
}

#[get("/history")]
fn history(state: &State<Mutex<AlarmState>>) -> String {
    let alarm = state.lock().unwrap();
    let mut values = Vec::new();
    for entry in alarm.history().entries().iter().rev() {
        values.push(serde_json::to_value(entry).unwrap());
    }
    serde_json::value::Value::Array(values).to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(AlarmState::default()))
        .mount("/", routes![status, signal, silence, history])
        .register("/", catchers![not_found, internal_error])
}
