use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    value: i32,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "ねこねこ" }</h1>
            </div>
        }
    }
}
