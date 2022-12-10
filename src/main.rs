mod board;
mod game;

const BOARD_SIZE: usize = 3;

fn main() {
    let b = board::make_board(BOARD_SIZE);
    game::play(b);
}
