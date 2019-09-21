use model::task::{Task, TaskData};
use stdweb::traits::IEvent;
use stdweb::unstable::TryInto;
use stdweb::web::{FormData, FormDataEntry};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    name: String,
}

pub enum Msg {
    Submit(stdweb::web::event::SubmitEvent),
    HandleChange(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            name: "".to_string(),
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
                true
            }
            Msg::HandleChange(value) => {
                self.name = value;
                console!(log, &self.name);
                false
            }
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
