use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties, Callback};
use game_of_life_logic::types::*;

pub struct CellComponent {
    cell: Cell,
    row_index: usize,
    cell_index: usize,
    on_cell_click: Callback<(usize, usize)>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    CellClick(usize, usize)
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub cell: Cell,
    pub row_index: usize,
    pub cell_index: usize,
    pub on_cell_click: Callback<(usize, usize)>,
}

impl Component for CellComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CellComponent {
            cell: props.cell,
            row_index: props.row_index,
            cell_index: props.cell_index,
            on_cell_click: props.on_cell_click,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CellClick(row, cell) => {
                self.cell.toggle();
                self.on_cell_click.emit((row,cell));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let state_changed = self.cell != props.cell;
        if state_changed {
            self.cell = props.cell;
        }
        
        state_changed
    }

    fn view(&self) -> Html {
        let class_name = match self.cell {
            Cell::Alive => "alive",
            Cell::Dead => "dead"
        };
        let row_index = self.row_index;
        let cell_index = self.cell_index;

        html! {
            <td class=class_name
                onclick=self.link.callback(move |_| Msg::CellClick(row_index, cell_index))></td>
        }
    }
}

