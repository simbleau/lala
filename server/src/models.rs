use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub enum HistoryKind {
    #[serde(rename = "responder")]
    ResponderCheckin,
    #[serde(rename = "alarm")]
    AlarmCheckin,
    #[serde(rename = "signal")]
    Signal,
    #[serde(rename = "silence")]
    Silence,
}

#[derive(Serialize)]
pub enum AlarmStatus {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

#[derive(Serialize)]
pub struct Host {
    pub addr: String,
    #[serde(serialize_with = "date_serialize")]
    pub last_checkin: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct HistoryEntry {
    pub id: i32,
    pub addr: String,
    pub kind: HistoryKind,
    #[serde(serialize_with = "date_serialize")]
    pub date: DateTime<Utc>,
}

fn date_serialize<S>(date: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&date.format("%d%m%Y_%H-%M-%S").to_string())
}

#[cfg(test)]
#[test]
fn name_test() {
    let on = AlarmStatus::On;
    assert_eq!(serde_json::to_string(&on).unwrap(), "on");

    let off = AlarmStatus::Off;
    assert_eq!(serde_json::to_string(&off).unwrap(), "off");
}
