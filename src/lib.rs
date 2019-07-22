#[macro_use]
extern crate stdweb;
extern crate log;
extern crate serde;
extern crate yew;

use log::info;
use yew::agent::Bridged;
use yew::{html, Bridge, Component, ComponentLink, Html, Renderable, ShouldRender};

pub mod assets;
use assets::asset;

pub mod component;
pub mod firebase;
pub mod route;
pub mod route_service;
pub mod router;

use route::Route;

pub enum Child {
    Tasks,
    TaskNew,
    PathNotFound(String),
}

pub struct Model {
    child: Child,
    router: Box<Bridge<router::Router<()>>>,
}

pub enum Msg {
    Click,
    NavigateTo(Child),
    HandleRoute(Route<()>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let mut router = router::Router::bridge(callback);
        router.send(router::Request::GetCurrentRoute);
        Model {
            child: Child::Tasks,
            router,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NavigateTo(child) => {
                let path_segments = match child {
                    Child::Tasks => vec!["tasks".into()],
                    Child::TaskNew => vec!["tasks".into(), "new".into()],
                    Child::PathNotFound(_) => vec!["path_not_fount".into()],
                };
                let route = Route {
                    path_segments,
                    query: None,
                    fragment: None,
                    state: (),
                };
                self.router.send(router::Request::ChangeRoute(route));
                false
            }
            Msg::HandleRoute(route) => {
                info!("Routing: {}", route.to_route_string());
                self.child = if let Some(first_segment) = route.path_segments.get(0) {
                    match first_segment.as_str() {
                        "tasks" => {
                            if let Some(second_segment) = route.path_segments.get(1) {
                                match second_segment.as_str() {
                                    "new" => Child::TaskNew,
                                    other => Child::PathNotFound(other.into()),
                                }
                            } else {
                                Child::Tasks
                            }
                        }
                        other => Child::PathNotFound(other.into()),
                    }
                } else {
                    Child::PathNotFound("path_not_fount".into())
                };
                true
            }
            Msg::Click => true,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="body", >
              {self.child.view()}
            </div>
        }
    }
}

impl Renderable<Model> for Child {
    fn view(&self) -> Html<Model> {
        match *self {
            Child::Tasks => html! {
                <component::tasks::Model: />
            },
            Child::TaskNew => html! {
                <>
                  {"route tasks/new"}
                </>
            },
            Child::PathNotFound(ref path) => html! {
                <>
                  <h1>{format!("Invalid path: '{}'", path)}</h1>
                  <button onclick=|_| Msg::NavigateTo(Child::Tasks),>{ "Go to /tasks" }</button>
                  <hr />
                  <button onclick=|_| Msg::Click,>{ "くりっく！" }</button>
                  <hr />
                  <img src={asset("1.png")}, style="height: 200px", />
                </>
            },
        }
    }
}
