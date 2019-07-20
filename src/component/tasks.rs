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
    AddTask
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
                let docs = qs.docs();
                let tasks = docs
                    .iter()
                    .map(|doc| {
                        doc.data().try_into().unwrap()
                    })
                    .collect();
                self.tasks = tasks;
                console!(log, &self.tasks);
            }
            Msg::AddTask => {
                console!(log, "add task");
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="tasks", >
                <ul>
                    { for self.tasks.iter().map(|task| self.view_task(task)) }
                </ul>
                <button class="add-button", onclick=|_| Msg::AddTask, >
                  {"+"}
                </button>
            </div>
        }
    }
}

impl Model {
    fn view_task(&self, task: &Task) -> Html<Model> {
        html! {
            <li>
                {&task.name}
            </li>
        }
    }
}
