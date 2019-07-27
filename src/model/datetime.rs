use chrono::Datelike;
use chrono::TimeZone;
use chrono::Timelike;
use chrono::Utc;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::ops::{Deref, DerefMut};
use stdweb::unstable::TryInto;
use stdweb::web::Date;
use stdweb::Value;

#[derive(Debug)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

impl DateTime {
    pub fn new(x: chrono::DateTime<chrono::Utc>) -> Self {
        Self(x)
    }

    pub fn now() -> Self {
        let date = Date::new();
        let utc = Utc
            .ymd(
                date.get_full_year(),
                date.get_month() as u32,
                date.get_day() as u32,
            )
            .and_hms_milli(
                date.get_hours() as u32,
                date.get_minutes() as u32,
                date.get_seconds() as u32,
                date.get_milliseconds() as u32,
            );
        Self(utc)
    }
}

impl Deref for DateTime {
    type Target = chrono::DateTime<chrono::Utc>;
    fn deref(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }
}

impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut chrono::DateTime<chrono::Utc> {
        &mut self.0
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let date = Date::from_datetime(
            self.0.year(),
            self.0.month() as i32,
            self.0.day() as i32,
            self.0.hour() as i32,
            self.0.minute() as i32,
            self.0.second() as i32,
            (self.0.nanosecond() / 1000) as i32,
        );
        Value::Reference(date.try_into().unwrap()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer).unwrap();
        console!(log, &v);
        let seconds = (js! {
            return @{&v}.seconds;
        })
        .try_into()
        .unwrap();
        console!(log, format!("{:?}", &seconds));
        let nanoseconds = (js! {
            return @{&v}.nanoseconds;
        })
        .try_into()
        .unwrap();
        console!(log, format!("{:?}", &nanoseconds));
        let utc = Utc.timestamp(seconds, nanoseconds);
        console!(log, format!("{:?}", &utc));
        Ok(DateTime(utc))
    }
}
