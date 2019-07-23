use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use stdweb::web::FormData;
use stdweb::traits::IEvent;
use stdweb::unstable::TryInto;

pub struct Model {}

pub enum Msg {
    Submit(stdweb::web::event::SubmitEvent),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(event) => {
                event.prevent_default();
                let form: stdweb::web::Element = event.target().unwrap().try_into().unwrap();
                let form_data = FormData::from_element(&form).unwrap();
                form_data.get("name");
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="TaskNew", >
                <form onsubmit=|event| Msg::Submit(event), >
                    <input name="name", type="text", placeholder="内容", />
                    <button>{"OK"}</button>
                </form>
            </div>
        }
    }
}
