mod board;
use board::*;
fn main() {
    let mut board = Board::new(1);
    board.set_symbol(1, 1, 1);
    board.set_symbol(2, 2, -1);
    board.print_board();
}

