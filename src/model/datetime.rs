use chrono::TimeZone;
use chrono::Utc;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::ops::{Deref, DerefMut};
use stdweb::unstable::TryInto;
use stdweb::web::Date;
use stdweb::Value;

#[derive(Debug)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

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
        Value::Reference(Date::new().try_into().unwrap()).serialize(serializer)
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
