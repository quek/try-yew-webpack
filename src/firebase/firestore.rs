use model::task::Task;
use serde::{Deserialize, Serialize};
use stdweb::unstable::TryInto;
use stdweb::Value;
use yew::Callback;

pub struct Firestore(Value);
pub struct Collection(Value);
pub struct Document(Value);
pub struct QuerySnapshot(Value);
pub struct QueryDocumentSnapshot(Value);
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentReference(Value);

impl Firestore {
    pub fn new() -> Self {
        let x = js! {
            return firebase.firestore();
        };
        Self(x)
    }

    pub fn collection(&self, name: &str) -> Collection {
        let x = js! {
            return @{&self.0}.collection(@{name});
        };
        Collection::new(x)
    }
}

impl Collection {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn doc(&self, name: &str) -> Document {
        let x = js! {
            return @{&self.0}.doc(@{name});
        };
        Document::new(x)
    }

    pub fn get(&self, callback: Callback<QuerySnapshot>) {
        let cb = move |value: Value| {
            callback.emit(QuerySnapshot::new(value));
        };
        js! {
            const callback = @{cb};
            return @{&self.0}.get().then(x => {
                callback(x);
                callback.drop();
            });
        };
    }

    pub fn add(&self, x: &Task) {
        js! {
            const data = {
                ...@{&x.data},
                created_at: @{&x.created_at},
                updated_at: @{&x.updated_at},
            };
            return @{&self.0}.add(data);
        };
    }
}

impl Document {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn collection(&self, name: &str) -> Collection {
        let x = js! {
            return @{&self.0}.collection(@{name});
        };
        Collection::new(x)
    }
}

impl QuerySnapshot {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn size(&self) -> u32 {
        js!(
            return @{&self.0}.size;
        )
        .try_into()
        .unwrap()
    }

    pub fn docs(&self) -> Vec<QueryDocumentSnapshot> {
        let x = js! {
            return @{&self.0}.docs;
        };
        let v: Vec<Value> = x.try_into().unwrap();
        v.iter()
            .map(|x| QueryDocumentSnapshot::new(x.clone()))
            .collect()
    }
}

impl QueryDocumentSnapshot {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn get(&self, field_path: &str) -> Value {
        js!( return @{&self.0}.get(@{field_path}); )
    }

    pub fn data(&self) -> Value {
        js!( return @{&self.0}.data() )
    }

    pub fn r#ref(&self) -> DocumentReference {
        DocumentReference(js!(
            return @{&self.0}.ref;
        ))
    }
}
