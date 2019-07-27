use firebase::firestore::DocumentReference;
// use firebase::timestamp::Timestamp;
use super::datetime::DateTime;
use firebase::auth::current_user;
use firebase::firestore::Firestore;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Task {
    pub r#ref: Option<DocumentReference>,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

js_serializable!(Task);
js_deserializable!(Task);

impl Task {
    pub fn create(&self) {
        let user = current_user();
        let firestore = Firestore::new();
        firestore
            .collection("users")
            .doc(&user.uid())
            .collection("tasks")
            .add(self);
    }
}
