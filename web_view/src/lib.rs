#![recursion_limit = "256"]

#[path="./universe_component.rs"]
pub mod universe_component;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use universe_component::UniverseComponent;

pub struct App {
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="center">
                <h1>{"Game of life"}</h1>
                <UniverseComponent />
            </div>
        }
    }
}
