extern crate piston_window;
extern crate rand;

use piston_window::{PistonWindow, WindowSettings};

mod draw;
mod snake;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            draw::draw_block([1.0, 0.0, 0.0, 1.0], 0, 0, &context, graphics);
        });
    }
}
