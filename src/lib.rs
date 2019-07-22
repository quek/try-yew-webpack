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
pub mod routing;

use routing::path::Path;
use routing::route::Route;

pub struct Model {
    route: Route,
    router: Box<Bridge<routing::router::Router>>,
}

pub enum Msg {
    Click,
    NavigateTo(Route),
    HandleRoute(Path),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|path: Path| Msg::HandleRoute(path));
        let mut router = routing::router::Router::bridge(callback);
        router.send(routing::router::Request::GetCurrentRoute);
        Model {
            route: Route::Tasks,
            router,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NavigateTo(route) => {
                let path_segments = match route {
                    Route::Tasks => vec!["tasks".into()],
                    Route::TaskNew => vec!["tasks".into(), "new".into()],
                    Route::PathNotFound(_) => vec!["path_not_fount".into()],
                };
                let route = Path {
                    path_segments,
                    query: None,
                    fragment: None,
                };
                self.router.send(routing::router::Request::ChangeRoute(route));
                false
            }
            Msg::HandleRoute(path) => {
                info!("Routing: {}", path.to_route_string());
                self.route = if let Some(first_segment) = path.path_segments.get(0) {
                    match first_segment.as_str() {
                        "tasks" => {
                            if let Some(second_segment) = path.path_segments.get(1) {
                                match second_segment.as_str() {
                                    "new" => Route::TaskNew,
                                    other => Route::PathNotFound(other.into()),
                                }
                            } else {
                                Route::Tasks
                            }
                        }
                        other => Route::PathNotFound(other.into()),
                    }
                } else {
                    Route::PathNotFound("path_not_fount".into())
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
              {self.route.view()}
            </div>
        }
    }
}

impl Renderable<Model> for Route {
    fn view(&self) -> Html<Model> {
        match *self {
            Route::Tasks => html! {
                <component::tasks::Model: />
            },
            Route::TaskNew => html! {
                <component::task_new::Model: />
            },
            Route::PathNotFound(ref path) => html! {
                <>
                  <h1>{format!("Invalid path: '{}'", path)}</h1>
                  <button onclick=|_| Msg::NavigateTo(Route::Tasks),>{ "Go to /tasks" }</button>
                  <hr />
                  <button onclick=|_| Msg::Click,>{ "くりっく！" }</button>
                  <hr />
                  <img src={asset("1.png")}, style="height: 200px", />
                </>
            },
        }
    }
}
