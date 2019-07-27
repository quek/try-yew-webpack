use stdweb::unstable::TryInto;
use stdweb::Reference;
use stdweb::ReferenceType;

// firebase.firestore.Timestamp
// https://firebase.google.com/docs/reference/js/firebase.firestore.Timestamp
/*
pub struct Timestamp {
    pub seconds: i64,
    pub nanoseconds: u32
}
*/

#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "firebase.firestore.Timestamp")]
pub struct Timestamp(Reference);

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

impl Timestamp {
    pub fn now() -> Self {
        js!(
            return new firebase.firestore.Timestamp.now();
        )
        .try_into()
        .unwrap()
    }
}
