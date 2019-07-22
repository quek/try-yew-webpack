use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
}

js_serializable!(Task);
js_deserializable!(Task);
