#![recursion_limit = "256"]

#[path="./universe_component.rs"]
pub mod universe_component;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use universe_component::UniverseComponent;

pub struct App {
}

pub enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                true
            }
        }
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