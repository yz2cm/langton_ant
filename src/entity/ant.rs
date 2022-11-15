use crate::common::{rule, unit};

pub struct Ant {
    pub direction: unit::Direction,
    pub x: usize,
    pub y: usize,
}
impl Ant {
    pub fn new(direction: unit::Direction, x: usize, y: usize) -> Self {
        Self {
            direction,
            x,
            y,
        }
    }
}
impl Ant {
    pub fn rotate(self, white_or_black: unit::CellColor) -> Self {
        Self {
            direction: rule::rotate_rule(white_or_black, self.direction),
            ..self
        }
    }
    pub fn move_forward(self) -> Self {
        let (x, y) = rule::movement_rule(self.direction, self.x, self.y);
        Self {
            x,
            y,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_test() {
        let actual = Ant { direction: unit::Direction::Up, x: 10, y: 20 };
        let actual = actual.rotate(unit::CellColor::White);
        assert_eq!(actual.direction, unit::Direction::Right);

        let actual = Ant { direction: unit::Direction::Right, x: 10, y: 20 };
        let actual = actual.rotate(unit::CellColor::White);
        assert_eq!(actual.direction, unit::Direction::Down);

        let actual = Ant { direction: unit::Direction::Down, x: 10, y: 20 };
        let actual = actual.rotate(unit::CellColor::White);
        assert_eq!(actual.direction, unit::Direction::Left);

        let actual = Ant { direction: unit::Direction::Left, x: 10, y: 20 };
        let actual = actual.rotate(unit::CellColor::White);
        assert_eq!(actual.direction, unit::Direction::Up);

        let actual = Ant { direction: unit::Direction::Up, x: 10, y: 20 };
        let actual = actual.rotate(unit::CellColor::Black);
        assert_eq!(actual.direction, unit::Direction::Left);

        let actual = Ant { direction: unit::Direction::Left, x: 10, y: 20 };
        let actual = actual.rotate(unit::CellColor::Black);
        assert_eq!(actual.direction, unit::Direction::Down);

        let actual = Ant { direction: unit::Direction::Down, x: 10, y: 20 };
        let actual = actual.rotate(unit::CellColor::Black);
        assert_eq!(actual.direction, unit::Direction::Right);

        let actual = Ant { direction: unit::Direction::Right, x: 10, y: 20 };
        let actual = actual.rotate(unit::CellColor::Black);
        assert_eq!(actual.direction, unit::Direction::Up);
    }
    #[test]
    fn move_foward_test() {
        let actual = Ant { direction: unit::Direction::Up, x: 10, y: 20 };
        let actual = actual.move_forward();
        assert_eq!((actual.x, actual.y), (10, 19));

        let actual = Ant { direction: unit::Direction::Right, x: 10, y: 20 };
        let actual = actual.move_forward();
        assert_eq!((actual.x, actual.y), (11, 20));

        let actual = Ant { direction: unit::Direction::Down, x: 10, y: 20 };
        let actual = actual.move_forward();
        assert_eq!((actual.x, actual.y), (10, 21));

        let actual = Ant { direction: unit::Direction::Left, x: 10, y: 20 };
        let actual = actual.move_forward();
        assert_eq!((actual.x, actual.y), (9, 20));
    }
}
