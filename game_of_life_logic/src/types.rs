#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universe {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>
}
