#[path="./cell_component.rs"]
pub mod cell_component;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use std::time::Duration;
use yew::services::{IntervalService, Task};
use game_of_life_logic::types::*;
use cell_component::CellComponent;

pub struct UniverseComponent {
    is_started: bool,
    universe: Universe,
    link: ComponentLink<Self>,
    job: Box<dyn Task>,
}

pub enum Msg {
    StepClick,
    StartClick,
    StopClick,
    ClearUniverse,
    Tick,
    OnCellClick(usize, usize)
}

impl Component for UniverseComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut universe = Universe::new(50, 50);
        universe.toggle_cells_state(vec![100,101,102,52,1]);
        // universe.toggle_cells_state(vec![1,2,3,4,5]);

        let callback = link.callback(|_| Msg::Tick);
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_millis(300), callback);

        UniverseComponent {
            is_started: false,
            universe,
            link,
            job: Box::new(handle),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnCellClick(row, cell) => {
                let index = self.universe.width * row + cell;
                self.universe.toggle_cells_state(vec![index]);
                return false;
            },
            Msg::StepClick => {
                self.next_generation();
                return true;
            },
            Msg::StartClick => {
                self.is_started = true;
                return true;
            },
            Msg::StopClick => {
                self.is_started = false;
                return true;
            },
            Msg::ClearUniverse => {
                self.is_started = false;
                self.universe.clear_universe();
                return true;
            },
            Msg::Tick => {
                if self.is_started {
                    self.next_generation();
                    return true;
                }
                return false;
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="center">
            // <p>{self.universe.cells.iter().position(|x| *x == Cell::Alive).unwrap().to_string()}</p>
                <table>
                    {
                        self.universe.cells.chunks(self.universe.width)
                            .enumerate()
                            .map(|(row_index, row_cells)| {
                                self.view_row(row_index, row_cells)
                            }).collect::<Html>()
                    }
                </table>
                <div>
                    <button onclick=self.link.callback(|_| Msg::StepClick)
                        disabled=self.is_started>{"Step"}</button>
                    <button onclick=self.link.callback(|_| Msg::StartClick)
                        disabled=self.is_started>{"Start"}</button>
                    <button onclick=self.link.callback(|_| Msg::StopClick)
                        disabled=!self.is_started>{"Stop"}</button>
                    <button onclick=self.link.callback(|_| Msg::ClearUniverse)
                        >{"Clear"}</button>
                </div>
            </div>
        }
    }
}

impl UniverseComponent {
    fn view_row(&self, row_index: usize, cells: &[Cell]) -> Html {
        html! {
            <tr>
                {cells.iter().enumerate().map(|(cell_index, cell)| {
                        self.view_cell(row_index, cell_index, *cell)
                    }).collect::<Html>()
                } 
            </tr>
        }
    }
    
    fn view_cell(&self, row_index:usize, cell_index: usize, cell: Cell) -> Html {
        html! {
            <CellComponent cell=cell
                cell_index=cell_index
                row_index=row_index
                on_cell_click=self.link.callback(move |(row_index, cell_index)| Msg::OnCellClick(row_index, cell_index))/>
        }
    }

    fn next_generation(&mut self) {
        let previous_generation = self.universe.clone();
        self.universe.next_generation();

        if self.universe == previous_generation {
            self.is_started = false;
        }
    }
}
