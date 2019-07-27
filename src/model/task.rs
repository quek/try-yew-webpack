use firebase::firestore::DocumentReference;
// use firebase::timestamp::Timestamp;
use super::datetime::DateTime;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Task {
    pub r#ref: Option<DocumentReference>,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

js_serializable!(Task);
js_deserializable!(Task);
