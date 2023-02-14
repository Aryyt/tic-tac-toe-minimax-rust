mod board;
use board::*;
fn main() {
    let mut board = Board::new(1);
    board.set_symbol(1, 1);
    board.set_symbol(3, -1);
    board.set_symbol(2, 1);
    board.print_board();
    let transpose = board.get_colls();
    printArray(&transpose[0]);
    let winner = board.check_arr_for_winner(&transpose[0]);
    println!();
    println!("{}", winner);
}

fn printArray(arr: &[i8]) {
    for i in arr {
        print!("{}", i);
    }
}
