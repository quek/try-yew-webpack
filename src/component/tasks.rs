use firebase::auth::current_user;
use firebase::firestore::Firestore;
use firebase::firestore::QuerySnapshot;
use serde::Deserialize;
use serde::Serialize;
use stdweb::unstable::TryInto;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    tasks: Vec<Task>,
}

pub enum Msg {
    GetTasks(QuerySnapshot),
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
}

js_serializable!(Task);
js_deserializable!(Task);

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let user = current_user();
        console!(log, user.uid());

        let callback = link.send_back(|qs: QuerySnapshot| Msg::GetTasks(qs));
        let firestore = Firestore::new();
        firestore
            .collection("users")
            .doc(&user.uid())
            .collection("tasks")
            .get(callback);

        Self { tasks: vec![] }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetTasks(qs) => {
                let size = qs.size();
                let docs = qs.docs();
                let tasks = docs
                    .iter()
                    .map(|doc| {
                        let task: Task = doc.data().try_into().unwrap();
                        task
                    })
                    .collect();
                self.tasks = tasks;
                console!(log, &self.tasks);

                console!(log, size);
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "ねこねこ " }</h1>
            </div>
        }
    }
}
