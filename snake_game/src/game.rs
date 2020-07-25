use piston_window::{clear, rectangle, Context, G2d, Key};
use rand::{thread_rng, Rng};

use crate::snake::*;

const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const GAME_OVER_BACKGROUND_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BORDER_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const APPLE_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const SNAKE_COLOR: [f32; 4] = [0.0, 0.8, 0.0, 1.0];

const WIDTH: i32 = 30;
const HEIGHT: i32 = 30;
const BLOCK_SIZE: f64 = 10.0;

const FPS: f64 = 10.0;

pub struct Game {
    width: i32,
    height: i32,

    snake: Snake,
    apple: (i32, i32),

    waiting_time: f64,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            snake: Snake::new(3, 1),
            apple: (10, 10),
            waiting_time: 0.0,
            game_over: false,
        }
    }

    pub fn width(&self) -> f64 {
        self.width as f64 * BLOCK_SIZE
    }

    pub fn height(&self) -> f64 {
        self.height as f64 * BLOCK_SIZE
    }
}

impl Game {
    pub fn handle_key(&mut self, key: Key) {
        self.snake.change_direction(match key {
            Key::Right => Direction::Right,
            Key::Left => Direction::Left,
            Key::Up => Direction::Up,
            Key::Down => Direction::Down,
            _ => return,
        });
    }
}

impl Game {
    pub fn render(&mut self, ctx: &Context, g: &mut G2d) {
        if self.game_over {
            clear(GAME_OVER_BACKGROUND_COLOR, g);
            return;
        }

        clear(BACKGROUND_COLOR, g);

        self.render_border(ctx, g);

        rectangle(
            APPLE_COLOR,
            self.to_gui_block(self.apple.0, self.apple.1, 1, 1),
            ctx.transform,
            g,
        );

        let draw_rectangle = |x: i32, y: i32| {
            rectangle(SNAKE_COLOR, self.to_gui_block(x, y, 1, 1), ctx.transform, g)
        };

        self.snake.draw(draw_rectangle);
    }

    pub fn render_border(&self, ctx: &Context, g: &mut G2d) {
        rectangle(
            BORDER_COLOR,
            self.to_gui_block(0, 0, 1, HEIGHT),
            ctx.transform,
            g,
        );
        rectangle(
            BORDER_COLOR,
            self.to_gui_block(WIDTH - 1, 0, 1, HEIGHT - 1),
            ctx.transform,
            g,
        );

        rectangle(
            BORDER_COLOR,
            self.to_gui_block(0, 0, WIDTH, 1),
            ctx.transform,
            g,
        );
        rectangle(
            BORDER_COLOR,
            self.to_gui_block(0, HEIGHT - 1, WIDTH, 1),
            ctx.transform,
            g,
        );
    }

    pub fn update(&mut self, delta: f64) {
        self.waiting_time += delta;

        if self.waiting_time > 1.0 / FPS {
            if self.game_over {
                self.restart();
            }

            self.snake.go_forward();

            let (head_x, head_y) = self.snake.head_position();

            if self.is_game_over() {
                self.game_over = true;
            }

            if head_x == self.apple.0 && head_y == self.apple.1 {
                self.snake.grow();
                self.apple = self.generate_random_apple();
            }

            self.waiting_time = 0.0;
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(3, 1);
        self.apple = self.generate_random_apple();

        self.waiting_time = 0.0;
        self.game_over = false;
    }

    fn is_game_over(&self) -> bool {
        let (head_x, head_y) = self.snake.head_position();

        self.snake.is_collided_with_body()
            || self.is_collided_with_border(head_x, head_y)
            || ((self.width - 2) * (self.height - 2)) as usize == self.snake.length()
    }

    fn generate_random_apple(&self) -> (i32, i32) {
        let mut rng = thread_rng();
        loop {
            let apple = (rng.gen_range(1, WIDTH - 1), rng.gen_range(1, HEIGHT - 1));

            if !self.snake.is_on_position(apple.0, apple.1) {
                return apple;
            }
        }
    }

    fn is_collided_with_border(&self, x: i32, y: i32) -> bool {
        x <= 0 || x >= HEIGHT - 1 || y <= 0 || y >= WIDTH - 1
    }

    fn to_gui_block(&self, x: i32, y: i32, w: i32, h: i32) -> [f64; 4] {
        [
            self.to_gui_coord(x),
            self.to_gui_coord(y),
            self.to_gui_coord(w),
            self.to_gui_coord(h),
        ]
    }

    fn to_gui_coord(&self, x: i32) -> f64 {
        x as f64 * BLOCK_SIZE
    }
}
