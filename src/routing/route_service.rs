use stdweb::web::event::PopStateEvent;
use stdweb::web::window;
use stdweb::web::EventListenerHandle;
use stdweb::web::History;
use stdweb::web::IEventTarget;
use stdweb::web::Location;
use yew::callback::Callback;

pub struct RouteService {
    history: History,
    location: Location,
    event_listener: Option<EventListenerHandle>,
}

impl RouteService {
    pub fn new() -> RouteService {
        let location = window()
            .location()
            .expect("browser does not support location API");
        RouteService {
            history: window().history(),
            location,
            event_listener: None,
        }
    }

    pub fn register_callback(&mut self, callback: Callback<String>) {
        self.event_listener = Some(window().add_event_listener(move |_: PopStateEvent| {
            let location: Location = window().location().unwrap();
            let route: String = Self::get_route_from_location(&location);
            callback.emit(route.clone())
        }));
    }

    pub fn push_state(&mut self, route: &str) {
        self.history.push_state((), "", Some(route));
    }

    fn get_route_from_location(location: &Location) -> String {
        let path = location.pathname().unwrap();
        let query = location.search().unwrap();
        let fragment = location.hash().unwrap();
        format!(
            "{path}{query}{fragment}",
            path = path,
            query = query,
            fragment = fragment
        )
    }

    pub fn get_route(&self) -> String {
        Self::get_route_from_location(&self.location)
    }

    pub fn get_path(&self) -> String {
        self.location.pathname().unwrap()
    }

    pub fn get_query(&self) -> String {
        self.location.search().unwrap()
    }

    pub fn get_fragment(&self) -> String {
        self.location.hash().unwrap()
    }
}
