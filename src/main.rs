mod board;
mod game;

use crate::board::make_board;

const BOARD_SIZE: usize = 3;

fn main() {
    let b = make_board(BOARD_SIZE);
    if !game::play(b) {
        println!("Sorry to see you go!");
    }
}
