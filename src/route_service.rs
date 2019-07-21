use std::marker::PhantomData;
use stdweb::unstable::TryFrom;
use stdweb::web::event::PopStateEvent;
use stdweb::web::window;
use stdweb::web::EventListenerHandle;
use stdweb::web::History;
use stdweb::web::IEventTarget;
use stdweb::web::Location;
use stdweb::JsSerialize;
use stdweb::Value;
use yew::callback::Callback;

pub struct RouteService<T> {
    history: History,
    location: Location,
    event_listener: Option<EventListenerHandle>,
    phantom_data: PhantomData<T>,
}

impl<T> RouteService<T>
where
    T: JsSerialize + Clone + TryFrom<Value> + 'static,
{
    pub fn new() -> RouteService<T> {
        let location = window()
            .location()
            .expect("browser does not support location API");
        RouteService {
            history: window().history(),
            location,
            event_listener: None,
            phantom_data: PhantomData,
        }
    }

    pub fn register_callback(&mut self, callback: Callback<(String, T)>) {
        self.event_listener = Some(window().add_event_listener(move |event: PopStateEvent| {
            let state_value: Value = event.state();
            if let Ok(state) = T::try_from(state_value) {
                let location: Location = window().location().unwrap();
                let route: String = Self::get_route_from_location(&location);
                callback.emit((route.clone(), state.clone()))
            } else {
                eprintln!("Nothing farther back in history, not calling routing callback.");
            }
        }));
    }

    pub fn set_route(&mut self, route: &str, state: T) {
        self.history.push_state(state, "", Some(route));
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
