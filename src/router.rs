use crate::route_service::RouteService;
use log::info;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;
use std::fmt::Debug;
use stdweb::unstable::TryFrom;
use stdweb::JsSerialize;
use stdweb::Value;
use yew::worker::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Route<T> {
    pub path_segments: Vec<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
    pub state: T,
}

impl<T> Route<T>
where
    T: JsSerialize + Clone + TryFrom<Value> + Default + 'static,
{
    pub fn to_route_string(&self) -> String {
        let path = self.path_segments.join("/");
        let mut path = format!("/{}", path);
        if let Some(ref query) = self.query {
            path = format!("{}?{}", path, query);
        }
        if let Some(ref fragment) = self.fragment {
            path = format!("{}#{}", path, fragment);
        }
        path
    }

    pub fn current_route(route_service: &RouteService<T>) -> Self {
        let path = route_service.get_path();
        let mut path_segments: Vec<String> = path.split("/").map(String::from).collect();
        path_segments.remove(0);

        let mut query: String = route_service.get_query();
        let query: Option<String> = if query.len() > 1 {
            query.remove(0);
            Some(query)
        } else {
            None
        };

        let mut fragment: String = route_service.get_fragment();
        let fragment: Option<String> = if fragment.len() > 1 {
            fragment.remove(0);
            Some(fragment)
        } else {
            None
        };

        Route {
            path_segments,
            query,
            fragment,
            state: T::default(),
        }
    }
}

pub enum Msg<T>
where
    T: JsSerialize + Clone + Debug + TryFrom<Value> + 'static,
{
    BrowserNavigationRouteChanged((String, T)),
}

impl<T> Transferable for Route<T> where for<'de> T: Serialize + Deserialize<'de> {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request<T> {
    ChangeRoute(Route<T>),
    ChangeRouteNoBroadcast(Route<T>),
    GetCurrentRoute,
}

impl<T> Transferable for Request<T> where for<'de> T: Serialize + Deserialize<'de> {}

pub struct Router<T>
where
    for<'de> T: JsSerialize
        + Clone
        + Debug
        + TryFrom<Value>
        + Default
        + Serialize
        + Deserialize<'de>
        + 'static,
{
    link: AgentLink<Router<T>>,
    route_service: RouteService<T>,
    subscribers: HashSet<HandlerId>,
}

impl<T> Agent for Router<T>
where
    for<'de> T: JsSerialize
        + Clone
        + Debug
        + TryFrom<Value>
        + Default
        + Serialize
        + Deserialize<'de>
        + 'static,
{
    type Reach = Context;
    type Message = Msg<T>;
    type Input = Request<T>;
    type Output = Route<T>;

    fn create(link: AgentLink<Self>) -> Self {
        console!(log, "Router create");
        let callback = link.send_back(|route_changed: (String, T)| {
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
            Msg::BrowserNavigationRouteChanged((_route_string, state)) => {
                info!("Browser navigated");
                let mut route = Route::current_route(&self.route_service);
                route.state = state;
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
                console!(log, format!("ChangeRoute {:?}", &route));
                let route_string: String = route.to_route_string();
                self.route_service.set_route(&route_string, route.state);
                let route = Route::current_route(&self.route_service);
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, route.clone());
                }
            }
            Request::ChangeRouteNoBroadcast(route) => {
                let route_string: String = route.to_route_string();
                self.route_service.set_route(&route_string, route.state);
            }
            Request::GetCurrentRoute => {
                let route = Route::current_route(&self.route_service);
                console!(log, format!("GetCurrentRoute {:?}", &route));
                self.link.response(who, route.clone());
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
