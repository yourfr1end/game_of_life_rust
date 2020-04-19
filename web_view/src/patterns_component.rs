use std::fmt;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties, Callback};
use yew::components::Select;

pub struct PatternsComponent {
    on_pattern_changed: Callback<Vec<(usize, usize)>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    OnSelectChange(Pattern),
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub on_pattern_changed: Callback<Vec<(usize, usize)>>,
}

impl Component for PatternsComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        PatternsComponent {
            on_pattern_changed: props.on_pattern_changed,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnSelectChange(pattern) => {
                self.on_pattern_changed.emit(pattern.indices)
            }
        }

        false
    }

    fn view(&self) -> Html {
        let patterns = get_default_patterns();
        let selected_pattern = patterns.first().unwrap().clone();

        html! {
            <div class="patterns_select">
                <Select<Pattern>
                    selected=selected_pattern
                    options=patterns
                    onchange=self.link.callback(Msg::OnSelectChange) />
            </div>
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Pattern {
    name: String,
    indices: Vec<(usize, usize)>,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn get_default_patterns() -> Vec<Pattern> {
    vec![
        Pattern{name:String::from("Empty"), indices:vec![]},
        Pattern{name:String::from("Glider"), indices:vec![(0,1),(1,2),(2,0),(2,1),(2,2)]},
        Pattern{name:String::from("Pulsar"), indices:vec![
            (0,2),(0,3),(0,9),(0,10),
            (1,3),(1,4),(1,8),(1,9),
            (2,0),(2,3),(2,5),(2,7),(2,9),(2,12),
            (3,0),(3,1),(3,2),(3,4),(3,5),(3,7),(3,8),(3,10),(3,11),(3,12),
            (4,1),(4,3),(4,5),(4,7),(4,9),(4,11),
            (5,2),(5,3),(5,4),(5,8),(5,9),(5,10),
            (7,2),(7,3),(7,4),(7,8),(7,9),(7,10),
            (8,1),(8,3),(8,5),(8,7),(8,9),(8,11),
            (9,0),(9,1),(9,2),(9,4),(9,5),(9,7),(9,8),(9,10),(9,11),(9,12),
            (10,0),(10,3),(10,5),(10,7),(10,9),(10,12),
            (11,3),(11,4),(11,8),(11,9),
            (12,2),(12,3),(12,9),(12,10),
        ]},
        Pattern{name:String::from("Space ship"), indices:vec![
            (0,1),(0,2),
            (1,0),(1,1),(1,2),(1,3),
            (2,0),(2,1),(2,3),(2,4),
            (3,2),(3,3),
        ]},
        Pattern{name:String::from("Quadpole"), indices:vec![
            (0,0),(0,1),
            (1,0),
            (2,1),(2,3),
            (4,3),(4,5),
            (5,6),
            (6,5),(6,6),
        ]},
        Pattern{name:String::from("Ten in a row"), indices:vec![
            (0,0),(0,1),(0,2),(0,3),(0,4),(0,5),(0,6),(0,7),(0,8),(0,9),
        ]},
        Pattern{name:String::from("Twenty in a row"), indices:vec![
            (0,0),(0,1),(0,2),(0,3),(0,4),(0,5),(0,6),(0,7),(0,8),(0,9),
            (0,10),(0,11),(0,12),(0,13),(0,14),(0,15),(0,16),(0,17),(0,18),(0,19),
        ]}
    ]
}
