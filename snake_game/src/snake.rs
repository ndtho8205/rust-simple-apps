use std::collections::VecDeque;

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
    length: usize,
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
            length: body.len(),
            direction: Direction::Right,
            body,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn is_collided_with_body(&self) -> bool {
        let (head_x, head_y) = self.head_position();

        for i in 1..self.body.len() {
            let block = &self.body[i];
            if block.x == head_x && block.y == head_y {
                return true;
            }
        }
        false
    }

    pub fn is_on_position(&self, x: i32, y: i32) -> bool {
        for block in &self.body {
            if block.x == x || block.y == y {
                return true;
            }
        }
        false
    }

    pub fn grow(&mut self) {
        if self.length() == self.body.len() {
            self.length += 1;
        }
    }

    pub fn change_direction(&mut self, d: Direction) {
        if self.direction.opposite() == d {
            return;
        }

        self.direction = d;
    }

    pub fn go_forward(&mut self) {
        let growth = self.length() != self.body.len();
        let (current_head_x, current_head_y) = self.head_position();

        let delta_head_position = match self.direction {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        };

        self.body.push_front(Block {
            x: current_head_x + delta_head_position.0,
            y: current_head_y + delta_head_position.1,
        });

        if !growth {
            self.body.pop_back();
        }
    }

    pub fn draw<F>(&self, mut draw_rectangle: F)
    where
        F: FnMut(i32, i32),
    {
        for block in &self.body {
            draw_rectangle(block.x, block.y);
        }
    }
}

#[cfg(test)]
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
    fn initial_snake_must_go_right() {
        assert_eq!(initial_snake().head_direction(), Direction::Right);
    }

    #[test]
    fn head_position_of_initial_snake_must_be_5_10() {
        check_head_position(&initial_snake(), 5, 10)
    }

    #[test]
    fn initial_snake_is_on_position_4_10() {
        assert_eq!(initial_snake().is_on_position(4, 10), true);
    }

    #[test]
    fn snake_can_only_grow_once_at_a_time() {
        let mut snake = initial_snake();
        snake.grow();
        snake.grow();
        assert_eq!(snake.length(), 4);
    }

    #[test]
    fn snake_go_right() {
        let mut snake = initial_snake();

        snake.change_direction(Direction::Right);
        assert_eq!(snake.head_direction(), Direction::Right);

        snake.go_forward();
        assert_eq!(snake.length(), 3);
        check_head_position(&snake, 6, 10);
    }

    #[test]
    fn snake_cannot_go_left() {
        let mut snake = initial_snake();

        snake.change_direction(Direction::Left);
        assert_eq!(snake.head_direction(), Direction::Right);

        assert_eq!(snake.length(), 3);
        check_head_position(&snake, 5, 10);
    }

    #[test]
    fn snake_go_up() {
        let mut snake = initial_snake();

        snake.change_direction(Direction::Up);
        assert_eq!(snake.head_direction(), Direction::Up);

        snake.go_forward();
        assert_eq!(snake.length(), 3);
        check_head_position(&snake, 5, 9);
    }

    #[test]
    fn snake_go_down() {
        let mut snake = initial_snake();

        snake.change_direction(Direction::Down);
        assert_eq!(snake.head_direction(), Direction::Down);

        snake.go_forward();
        assert_eq!(snake.length(), 3);
        check_head_position(&snake, 5, 11);
    }

    #[test]
    fn draw_initial_snake() {
        let snake = initial_snake();
        let mut drawed_point = Vec::<(i32, i32)>::new();

        let draw_rectangle = |x: i32, y: i32| drawed_point.push((x, y));
        snake.draw(draw_rectangle);

        assert_eq!(drawed_point.len(), 3);
        assert_eq!(drawed_point[0], (5, 10));
        assert_eq!(drawed_point[1], (4, 10));
        assert_eq!(drawed_point[2], (3, 10));
    }
}
