extern crate rand;
extern crate piston_window;

mod draw;
mod game;
mod tetris;

use piston_window::*;
use piston_window::types::Color;

use draw::{draw_block, to_coord_u32};
use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    // TODO: refactor this to a constant
    // COLUMN COUNT, ROW COUNT
    let (width, height) = (14, 24);

    let mut window: PistonWindow = WindowSettings::new(
        "Tetris",
        [to_coord_u32(width), to_coord_u32(height)]
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        // Get pressed key
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
    }
}
