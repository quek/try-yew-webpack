use firebase::auth::current_user;
use firebase::firestore::Firestore;
use firebase::firestore::QuerySnapshot;
use model::task::Task;
use routing::path::Path;
use routing::route::Route;
use routing::router::{Request, Router};
use stdweb::unstable::TryInto;
use yew::agent::Bridged;
use yew::{html, Bridge, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    tasks: Vec<Task>,
    router: Box<Bridge<Router>>,
}

pub enum Msg {
    GetTasks(QuerySnapshot),
    AddTask,
    HandleRoute(Path),
}

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

        let callback = link.send_back(|path: Path| Msg::HandleRoute(path));
        let router = Router::bridge(callback);
        Self {
            tasks: vec![],
            router,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetTasks(qs) => {
                let docs = qs.docs();
                let tasks = docs
                    .iter()
                    .map(|doc| {
                        let data = doc.data().try_into().unwrap();
                        Task {
                            r#ref: Some(doc.r#ref()),
                            data,
                        }
                    })
                    .collect();
                self.tasks = tasks;
                console!(log, format!("{:?}", &self.tasks));
                true
            }
            Msg::AddTask => {
                self.router.send(Request::ChangeRoute(Route::TaskNew));
                false
            }
            Msg::HandleRoute(_) => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="tasks">
                <ul>
                    { for self.tasks.iter().map(|task| self.view_task(task)) }
                </ul>
                <button class="add-button", onclick=|_| Msg::AddTask>
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
                {&task.data.name}
            </li>
        }
    }
}
