use crate::types::*;

impl Cell {
    pub fn toggle(&mut self){
        *self = match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}