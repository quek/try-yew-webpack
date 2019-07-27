use firebase::auth::current_user;
use firebase::firestore::DocumentReference;
use firebase::firestore::Firestore;
use firebase::timestamp::Timestamp;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TaskData {
    pub name: String,
}

js_serializable!(TaskData);
js_deserializable!(TaskData);

#[derive(Debug)]
pub struct Task {
    pub r#ref: Option<DocumentReference>,
    pub data: TaskData,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

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
