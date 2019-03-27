#[macro_use]
extern crate stdweb;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;

pub struct Model {
    console: ConsoleService,
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.console.log("にゃん♪");
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "はろ～ですよ" }</h1>
                <button onclick=|_| Msg::Click,>{ "くりっく！" }</button>
                <hr />
                <img src={assets("1.png")}, />
            </div>
        }
    }
}

fn assets(name: &'static str) -> std::string::String {
    (js! { return window.assets(@{name}); }).into_string().unwrap()
}
