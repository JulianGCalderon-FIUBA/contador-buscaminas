mod mine_counter;
use mine_counter::CountMines;

fn main() {
    let mine_counter = match CountMines::from_file("boards/big.txt") {
        Ok(mine_counter) => mine_counter,
        Err(err) => return eprintln!("Could not read board, with error: {:?}", err),
    };

    mine_counter.display();
}
