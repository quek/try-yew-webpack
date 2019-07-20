use firebase::auth::current_user;
use firebase::firestore::Firestore;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    value: i32,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let user = current_user();
        console!(log, user.uid());

        let firestore = Firestore::new();
        let _tasks = firestore
            .collection("users")
            .doc(&user.uid())
            .collection("tasks")
            .get();

        Model { value: 0 }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{ "ねこねこ " }{self.value}</h1>
            </div>
        }
    }
}
