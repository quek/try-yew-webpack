use stdweb::Value;

pub struct User(Value);

pub fn current_user() -> User {
    let user = js! {
        return firebase.auth().currentUser;
    };
    User(user)
}

impl User {
    pub fn uid(&self) -> String {
        let uid = js! {
            return @{&self.0}.uid
        };
        uid.into_string().unwrap()
    }
}
