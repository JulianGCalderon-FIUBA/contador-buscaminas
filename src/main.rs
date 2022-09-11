use buscaminas::board::Board;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return eprintln!("Invalid arguements, should receive path to board file.");
    }

    let file_name = &args[1];

    let mut board = match Board::from_file(file_name) {
        Ok(board) => board,
        Err(err) => return eprintln!("Could not read board, with error: {:?}", err),
    };

    println!("Original Board:");
    board.display();

    board.count_mines();

    println!("Modified Board:");
    board.display();

    if let Err(err) = board.to_file("boards/exported.txt") {
        eprintln!("Could not write board to file, with error: {:?}", err);
    };
}
