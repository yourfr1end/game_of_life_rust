use std::fmt;
use crate::types::*;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for chunk in self.cells.chunks(self.width) {
            for cell in chunk {
                 match cell {
                     Cell::Alive => write!(f, "{}", "x")?,
                     Cell::Dead => write!(f, "{}", "-")?
                 }
            }
            write!(f, "{}", "\n")?
        }

        write!(f, "{}", "")
    }
}
