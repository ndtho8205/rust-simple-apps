extern crate piston_window;
extern crate rand;

use piston_window::*;

mod game;
mod snake;

use game::Game;

fn main() {
    let mut game = Game::new();

    let mut window: PistonWindow = WindowSettings::new("Snake Game", [game.width(), game.height()])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        if let Some(key) = event.button_args() {
            if key.state == ButtonState::Press {
                match key.button {
                    Button::Keyboard(k) => game.handle_key(k),
                    _ => (),
                }
            }
        }

        window.draw_2d(&event, |context, graphics, _device| {
            game.render(&context, graphics);
        });

        if let Some(args) = event.update_args() {
            game.update(args.dt);
        }
    }
}
