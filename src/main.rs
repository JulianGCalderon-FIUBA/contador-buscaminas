mod mine_counter;
use mine_counter::MineCounter;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return eprintln!("Invalid arguements, should receive path to board file.");
    }

    let mine_counter = match MineCounter::from_file(&args[1]) {
        Ok(mine_counter) => mine_counter,
        Err(err) => return eprintln!("Could not read board, with error: {:?}", err),
    };

    mine_counter.display();
    if let Err(err) = mine_counter.to_file("boards/exported.txt") {
        eprintln!("Could not write board to file, with error: {:?}", err);
    };
}
