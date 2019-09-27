use model::task::{Task, TaskData};
use stdweb::traits::IEvent;
use stdweb::unstable::TryInto;
use stdweb::web::{FormData, FormDataEntry};
use yew::{html, Bridge, Component, ComponentLink, Html, Renderable, ShouldRender};
use routing::path::Path;
use routing::route::Route;
use routing::router::{Request, Router};
use yew::agent::Bridged;

pub struct Model {
    name: String,
    router: Box<Bridge<Router>>,
}

pub enum Msg {
    Submit(stdweb::web::event::SubmitEvent),
    HandleChange(String),
    HandleRoute(Path),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|path: Path| Msg::HandleRoute(path));
        let router = Router::bridge(callback);
        Self {
            name: "".to_string(),
            router
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(event) => {
                console!(log, "Submit");
                event.prevent_default();
                let form: stdweb::web::Element = event.target().unwrap().try_into().unwrap();
                let form_data = FormData::from_element(&form).unwrap();
                if let Some(FormDataEntry::String(name)) = form_data.get("name") {
                    let task = Task {
                        r#ref: None,
                        data: TaskData { name },
                    };
                    console!(log, format!("{:?}", &task));
                    task.create();
                }
                self.router.send(Request::ChangeRoute(Route::Tasks));
                true
            }
            Msg::HandleChange(value) => {
                self.name = value;
                console!(log, &self.name);
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
            <div class="TaskNew">
                <form onsubmit=|event| Msg::Submit(event)>
                    <input
                        name="name"
                        type="text"
                        placeholder="内容"
                        oninput=|event| Msg::HandleChange(event.value)
                    />
                    <button>{"OK"}</button>
                </form>
            </div>
        }
    }
}
