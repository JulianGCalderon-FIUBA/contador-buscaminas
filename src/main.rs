mod board;
use board::Board;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return eprintln!("Invalid arguements, should receive path to board file.");
    }

    let mut board = match Board::from_file(&args[1]) {
        Ok(board) => board,
        Err(err) => return eprintln!("Could not read board, with error: {:?}", err),
    };

    board.display();
    board.count_mines();
    board.display();

    if let Err(err) = board.to_file("boards/exported.txt") {
        eprintln!("Could not write board to file, with error: {:?}", err);
    };
}
