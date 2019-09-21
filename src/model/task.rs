use firebase::auth::current_user;
use firebase::firestore::DocumentReference;
use firebase::firestore::Firestore;
use serde::Serialize;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TaskData {
    pub name: String,
}

js_serializable!(TaskData);
js_deserializable!(TaskData);

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    #[serde(skip)]
    pub r#ref: Option<DocumentReference>,
    pub data: TaskData,
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
