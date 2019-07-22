use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
}

pub enum Msg {
    Submit
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {
                console!(log, "submit");
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
            <div class="tasks", >
                  {"新しいの"}
            </div>
        }
    }
}