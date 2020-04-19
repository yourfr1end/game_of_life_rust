#![recursion_limit = "256"]

pub mod universe_component;
pub mod patterns_component;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use universe_component::UniverseComponent;
use patterns_component::PatternsComponent;

pub struct App {
    selected_pattern: Vec<(usize, usize)>,
    link: ComponentLink<Self>
}

pub enum Msg {
    OnSelectChange(Vec<(usize, usize)>),
} 

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            selected_pattern: vec![],
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnSelectChange(indices) => {
                self.selected_pattern = indices;
            }
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="center">
                <h1>{"Game of life"}</h1>
                <PatternsComponent
                    on_pattern_changed=self.link.callback(move |indices| Msg::OnSelectChange(indices))/>
                <UniverseComponent selected_pattern=&self.selected_pattern />
            </div>
        }
    }
}
