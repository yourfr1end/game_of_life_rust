use crate::types::*;

impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        let cells = vec![Cell::Dead; width*height];
        Universe {
            width,
            height,
            cells
        }
    }

    pub fn toggle_cells_state(&mut self, indices: Vec<usize>) {
        for index in indices.into_iter().filter(|x| *x <= self.height*self.width)
                            .collect::<Vec<_>>() {
            self.cells[index].toggle();
        }
    }

    pub fn next_generation(&mut self) {
        let next_generation = self.cells.chunks(self.width).enumerate()
                .map(|(row_index, row_cells)| {
                    row_cells.iter().enumerate().map(|(cell_index, cell)| {
                        let alive_neighbours_count = self.count_alive_neighbours(cell_index, row_index);
                        match *cell {
                            Cell::Alive if alive_neighbours_count == 2 || alive_neighbours_count == 3 => Cell::Alive,
                            Cell::Dead if alive_neighbours_count == 3 => Cell::Alive,
                            _ => Cell::Dead
                        }
                    }).collect::<Vec<_>>()
                }).collect::<Vec<_>>().concat();

        self.cells = next_generation
    }

    pub fn clear_universe(&mut self) {
        self.cells = vec![Cell::Dead; self.width*self.height];
    }

    fn count_alive_neighbours(&self, cell_index: usize, row_index: usize) -> usize {
        self.get_neigbours(cell_index, row_index).iter()
            .fold(0, |sum, val| { 
                sum + match *val {
                    Cell::Alive => 1,
                    Cell::Dead => 0
                }
            })
    }

    fn get_neigbours(&self, cell_index: usize, row_index: usize) -> [Cell;8] {
        [
            self.get_cell(row_index as isize - 1, cell_index as isize - 1),
            self.get_cell(row_index as isize - 1, cell_index as isize),
            self.get_cell(row_index as isize - 1, cell_index as isize + 1),
            self.get_cell(row_index as isize, cell_index as isize - 1),
            self.get_cell(row_index as isize, cell_index as isize + 1),
            self.get_cell(row_index as isize + 1, cell_index as isize - 1),
            self.get_cell(row_index as isize + 1, cell_index as isize),
            self.get_cell(row_index as isize + 1, cell_index as isize + 1),
        ]
    }

    fn get_cell(&self, row_index: isize, cell_index: isize) -> Cell {
        let row_index = if row_index < 0 { 
            self.height - 1
        } else if row_index >= self.height as isize {
            0
        } else {
            row_index as usize
        };

        let cell_index = if cell_index < 0 {
            self.width - 1
        } else if cell_index >= self.width as isize {
            0
        } else {
            cell_index as usize
        };

        self.cells[self.width*row_index + cell_index]
    }
}
