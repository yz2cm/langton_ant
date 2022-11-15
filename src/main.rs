use langton_ant::entity::*;
use langton_ant::common::*;

fn main() {
    let mut ant = ant::Ant::new(unit::Direction::Left, 100, 100);
    let mut board = board::Board::new();

    for _ in 0..21000 {
        let white_or_black = board.ant_cell_color(&ant);
        ant = ant.rotate(white_or_black);
        board.turn_over_cell(&ant);
        ant = ant.move_forward();
    }
    board.print(&ant);
}
