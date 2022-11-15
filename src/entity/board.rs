use crate::common::{rule, unit};
use super::ant::Ant;

const WIDTH: usize = 500;
const HEIGHT: usize = 350;

#[derive(Clone)]
pub struct Board {
    board: [[unit::CellColor; WIDTH]; HEIGHT],
}
impl Board {
    pub fn new() -> Self {
        Self {
            board : [[unit::CellColor::White; WIDTH]; HEIGHT],
        }
    }
}
impl Board {
    pub fn turn_over_cell(&mut self, ant: &Ant) {
        self.board[ant.y][ant.x] = rule::turn_over_rule(self.board[ant.y][ant.x]);
    }
    pub fn print(&self, ant: &Ant) {
        let mut y = 0;
        for &cells in self.board.iter() {
            let mut x = 0;
            for cell in cells.into_iter() {
                print!("{}", rule::cell_to_symbol_rule(x, y, cell, ant));
                x += 1;
            } 
            println!("");
            y += 1;
        }
    }
    pub fn ant_cell_color(&self, ant: &Ant) -> unit::CellColor {
        self.board[ant.y][ant.x]
    }
}
