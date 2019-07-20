use stdweb::Value;

pub struct Firestore(Value);
pub struct Collection(Value);
pub struct Document(Value);
pub struct QuerySnapshot(Value);

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

    pub fn get(&self) -> QuerySnapshot {
        let x = js! {
            return @{&self.0}.get();
        };
        QuerySnapshot::new(x)
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
}
