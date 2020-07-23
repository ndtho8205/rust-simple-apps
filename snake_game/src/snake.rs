use std::collections::VecDeque;

const COLOR: [f64; 4] = [0.0, 0.8, 0.0, 1.0];

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: VecDeque<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body = VecDeque::<Block>::new();
        for i in 0..=2 {
            body.push_back(Block { x: x - i, y })
        }

        Self {
            direction: Direction::Right,
            body,
        }
    }

    pub fn length(&self) -> usize {
        self.body.len()
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn go_forward(&mut self, direction: Option<Direction>) {
        if let Some(d) = direction {
            if self.direction.opposite() == d {
                return;
            }

            self.direction = d;
        }

        let (current_head_x, current_head_y) = self.head_position();

        let new_head_block = match self.direction {
            Direction::Right => Block {
                x: current_head_x + 1,
                y: current_head_y,
            },
            Direction::Left => Block {
                x: current_head_x - 1,
                y: current_head_y,
            },
            Direction::Up => Block {
                x: current_head_x,
                y: current_head_y - 1,
            },
            Direction::Down => Block {
                x: current_head_x,
                y: current_head_y + 1,
            },
        };

        self.body.push_front(new_head_block);
        self.body.pop_back();
    }
}

mod test {
    use super::*;

    fn initial_snake() -> Snake {
        Snake::new(5, 10)
    }

    fn check_head_position(snake: &Snake, want_x: i32, want_y: i32) {
        let head_position = snake.head_position();
        assert_eq!(head_position.0, want_x);
        assert_eq!(head_position.1, want_y);
    }

    #[test]
    fn initial_snake_must_have_length_of_3() {
        assert_eq!(initial_snake().length(), 3);
    }

    #[test]
    fn new_snake_must_go_right() {
        assert_eq!(initial_snake().head_direction(), Direction::Right);
    }

    #[test]
    fn head_position_of_initial_snake_must_be_5_10() {
        check_head_position(&initial_snake(), 5, 10)
    }

    #[test]
    fn initial_snake_go_right() {
        let mut snake = initial_snake();
        snake.go_forward(Some(Direction::Right));

        assert_eq!(snake.length(), 3);
        assert_eq!(snake.head_direction(), Direction::Right);

        check_head_position(&snake, 6, 10);
    }

    #[test]
    fn initial_snake_cannot_go_left() {
        let mut snake = initial_snake();
        snake.go_forward(Some(Direction::Left));

        assert_eq!(snake.length(), 3);
        assert_eq!(snake.head_direction(), Direction::Right);

        check_head_position(&snake, 5, 10);
    }

    #[test]
    fn initial_snake_go_up() {
        let mut snake = initial_snake();
        snake.go_forward(Some(Direction::Up));

        assert_eq!(snake.length(), 3);
        assert_eq!(snake.head_direction(), Direction::Up);

        check_head_position(&snake, 5, 9);
    }

    #[test]
    fn initial_snake_go_down() {
        let mut snake = initial_snake();
        snake.go_forward(Some(Direction::Down));

        assert_eq!(snake.length(), 3);
        assert_eq!(snake.head_direction(), Direction::Down);

        check_head_position(&snake, 5, 11);
    }
}
