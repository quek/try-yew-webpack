#[macro_use]
extern crate stdweb;
extern crate yew;

use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub mod assets;
use assets::path;

pub mod component;
pub mod firebase;

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
                <component::tasks::Model: />
                <h1>{ "はろ～ですよ" }</h1>
                <button onclick=|_| Msg::Click,>{ "くりっく！" }</button>
                <hr />
                <img src={path("1.png")}, style="height: 200px", />
            </div>
        }
    }
}
