use crate::common::unit::*;
use crate::entity::ant::Ant;

pub fn rotate_rule(white_or_black: CellColor, direction: Direction) -> Direction {
    match white_or_black {
        CellColor::White => match direction {
            Direction::Up    => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
        },
        CellColor::Black => match direction {
            Direction::Up    => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down  => Direction::Right,
            Direction::Left  => Direction::Down,
        }
    }
}

pub fn movement_rule(direction: Direction, x: usize, y: usize) -> (usize, usize) {
    match direction {
        Direction::Up    => (x,   y-1),
        Direction::Right => (x+1, y),
        Direction::Down  => (x,   y+1),
        Direction::Left  => (x-1, y),
    }
}
pub fn turn_over_rule(white_or_black: CellColor) -> CellColor {
    match white_or_black {
        CellColor::White => CellColor::Black,
        CellColor::Black => CellColor::White,
    }
}
pub fn cell_to_symbol_rule(x: usize, y: usize, white_or_black: CellColor, ant: &Ant) -> char {
    match white_or_black {
        _ if x == ant.x && y == ant.y => '★',
        CellColor::Black => 'X',
        _ => '_',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_rule_test_white() {
        let actual = rotate_rule(CellColor::White, Direction::Up);
        assert_eq!(actual, Direction::Right);
        let actual = rotate_rule(CellColor::White, Direction::Right);
        assert_eq!(actual, Direction::Down);
        let actual = rotate_rule(CellColor::White, Direction::Down);
        assert_eq!(actual, Direction::Left);
        let actual = rotate_rule(CellColor::White, Direction::Left);
        assert_eq!(actual, Direction::Up);
    }
    #[test]
    fn rotate_rule_test_black() {
        let actual = rotate_rule(CellColor::Black, Direction::Up);
        assert_eq!(actual, Direction::Left);
        let actual = rotate_rule(CellColor::Black, Direction::Right);
        assert_eq!(actual, Direction::Up);
        let actual = rotate_rule(CellColor::Black, Direction::Down);
        assert_eq!(actual, Direction::Right);
        let actual = rotate_rule(CellColor::Black, Direction::Left);
        assert_eq!(actual, Direction::Down);
    }
    #[test]
    fn movement_rule_test() {
        let (actual_x, actual_y) = movement_rule(Direction::Up, 10, 20);
        assert_eq!(actual_x, 10);
        assert_eq!(actual_y, 19);

        let (actual_x, actual_y) = movement_rule(Direction::Right, 10, 20);
        assert_eq!(actual_x, 11);
        assert_eq!(actual_y, 20);

        let (actual_x, actual_y) = movement_rule(Direction::Down, 10, 20);
        assert_eq!(actual_x, 10);
        assert_eq!(actual_y, 21);

        let (actual_x, actual_y) = movement_rule(Direction::Left, 10, 20);
        assert_eq!(actual_x, 9);
        assert_eq!(actual_y, 20);
    }
    #[test]
    fn turn_over_rule_test() {
        let actual = turn_over_rule(CellColor::White);
        assert_eq!(actual, CellColor::Black);

        let actual = turn_over_rule(CellColor::Black);
        assert_eq!(actual, CellColor::White);
    }
    #[test]
    fn cell_to_symbol_rule_test() {
        let actual = cell_to_symbol_rule(10, 20, CellColor::White, &Ant { direction: Direction::Up, x: 10, y: 20 });
        assert_eq!(actual, '★');
        let actual = cell_to_symbol_rule(10, 20, CellColor::Black, &Ant { direction: Direction::Up, x: 10, y: 20 });
        assert_eq!(actual, '★');
        let actual = cell_to_symbol_rule(10, 20, CellColor::White, &Ant { direction: Direction::Up, x: 100, y: 200 });
        assert_eq!(actual, '_');
        let actual = cell_to_symbol_rule(10, 20, CellColor::White, &Ant { direction: Direction::Up, x: 10, y: 200 });
        assert_eq!(actual, '_');
        let actual = cell_to_symbol_rule(10, 20, CellColor::White, &Ant { direction: Direction::Up, x: 100, y: 20 });
        assert_eq!(actual, '_');
        let actual = cell_to_symbol_rule(10, 20, CellColor::Black, &Ant { direction: Direction::Up, x: 100, y: 200 });
        assert_eq!(actual, 'X');
        let actual = cell_to_symbol_rule(10, 20, CellColor::Black, &Ant { direction: Direction::Up, x: 10, y: 200 });
        assert_eq!(actual, 'X');
        let actual = cell_to_symbol_rule(10, 20, CellColor::Black, &Ant { direction: Direction::Up, x: 100, y: 20 });
        assert_eq!(actual, 'X');
    }
}