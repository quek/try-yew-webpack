use firebase::firestore::DocumentReference;
use stdweb::Reference;
use stdweb::ReferenceType;
use stdweb::unstable::TryInto;

#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Timestamp")]
pub struct Timestamp( Reference );

pub trait ITimestamp: ReferenceType {
    fn seconds(&self) -> i64 {
        js!(
            return @{self.as_ref()}.seconds;
        )
        .try_into()
        .unwrap()
    }
    fn nanoseconds(&self) -> u32 {
        js!(
            return @{self.as_ref()}.nanoseconds;
        )
        .try_into()
        .unwrap()
    }
}

impl ITimestamp for Timestamp {}

// firebase.firestore.Timestamp
// https://firebase.google.com/docs/reference/js/firebase.firestore.Timestamp
/*
pub struct Timestamp {
    pub seconds: i64, 
    pub nanoseconds: u32
}
*/

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Task {
    pub r#ref: Option<DocumentReference>,
    pub name: String,
    // pub created_at: Timestamp,
    // pub updated_at: Timestamp
}

js_serializable!(Task);
js_deserializable!(Task);
