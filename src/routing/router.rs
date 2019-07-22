use super::path::Path;
use super::route::Route;
use super::route_service::RouteService;
use log::info;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use yew::worker::*;

pub enum Msg {
    BrowserNavigationRouteChanged(String),
}

impl Transferable for Path {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    ChangeRoute(Route),
    ChangeRouteNoBroadcast(Path),
    GetCurrentRoute,
}

impl Transferable for Request {}

pub struct Router {
    link: AgentLink<Router>,
    route_service: RouteService,
    subscribers: HashSet<HandlerId>,
}

impl Agent for Router {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = Path;

    fn create(link: AgentLink<Self>) -> Self {
        console!(log, "Router create");
        let callback = link.send_back(|route_changed: String| {
            Msg::BrowserNavigationRouteChanged(route_changed)
        });
        let mut route_service = RouteService::new();
        route_service.register_callback(callback);

        Router {
            link,
            route_service,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        console!(log, format!("Router update {:?}", 111));
        match msg {
            Msg::BrowserNavigationRouteChanged(_route_string) => {
                info!("Browser navigated");
                let route = Path::current(&self.route_service);
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, route.clone());
                }
            }
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            Request::ChangeRoute(route) => {
                let path = route.to_path();
                console!(log, format!("ChangeRoute {:?}", &path));
                let route_string: String = path.to_route_string();
                self.route_service.push_state(&route_string);
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, path.clone());
                }
            }
            Request::ChangeRouteNoBroadcast(route) => {
                let route_string: String = route.to_route_string();
                self.route_service.push_state(&route_string);
            }
            Request::GetCurrentRoute => {
                let path = Path::current(&self.route_service);
                console!(log, format!("GetCurrentRoute {:?}", &path));
                self.link.response(who, path.clone());
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
