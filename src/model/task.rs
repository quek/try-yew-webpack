use firebase::firestore::DocumentReference;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Task {
    pub r#ref: Option<DocumentReference>,
    pub name: String,
    // pub created_at: Timestamp,
    // pub updated_at: Timestamp
}

js_serializable!(Task);
js_deserializable!(Task);
