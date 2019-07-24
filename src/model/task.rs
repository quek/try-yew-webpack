use serde::{Deserialize, Serialize};
use firebase::firestore::DocumentReference;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub r#ref: Option<DocumentReference>,
    pub name: String,
    pub created_at: stdweb::web::Date,
    pub updated_at: chrono::DateTime<chrono::Local>
}

js_serializable!(Task);
js_deserializable!(Task);
