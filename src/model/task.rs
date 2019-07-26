use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};
use firebase::firestore::DocumentReference;
use std::ops::{Deref, DerefMut};
use chrono::TimeZone;

// firebase.firestore.Timestamp
// https://firebase.google.com/docs/reference/js/firebase.firestore.Timestamp
pub struct Timestamp {
    pub seconds: i64, 
    pub nanoseconds: u32
}

#[derive(Debug)]
pub struct DateTime(chrono::DateTime<chrono::Local>);

impl Deref for DateTime {
    type Target = chrono::DateTime<chrono::Local>;
    fn deref(&self) -> &chrono::DateTime<chrono::Local> {
        &self.0
    }
}

impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut chrono::DateTime<chrono::Local> {
        &mut self.0
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = i32::deserialize(deserializer);
        Ok(DateTime(chrono::Local.timestamp(0, 0)))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Task {
    pub r#ref: Option<DocumentReference>,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime
}

js_serializable!(Task);
js_deserializable!(Task);
