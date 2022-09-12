use buscaminas::board::Board;
use buscaminas::input_error::InputError;

#[test]
fn valid_board_displays_correctly() {
    let mine_counter = Board::from_file("boards/basic.txt").unwrap();

    assert_eq!(".*.*.\n..*..\n..*..\n.....\n", format!("{}", mine_counter))
}

#[test]
fn valid_board_counts_mines_correctly() {
    let mut mine_counter = Board::from_file("boards/basic.txt").unwrap();
    mine_counter.count_mines();
    assert_eq!("1*3*1\n13*31\n.2*2.\n.111.\n", format!("{}", mine_counter))
}

#[test]
fn loading_board_with_invalid_character_returns_corresponding_error() {
    let mine_counter = Board::from_file("boards/invalid_character.txt");
    assert!(matches!(
        mine_counter,
        Err(InputError::InvalidCharacter('A'))
    ));
}

#[test]
fn loading_board_with_invalid_board_returns_corresponding_error() {
    let mine_counter = Board::from_file("boards/invalid_board.txt");
    assert!(matches!(mine_counter, Err(InputError::InvalidBoard)));
}

#[test]
fn loading_empty_board_returns_corresponding_error() {
    let mine_counter = Board::from_file("boards/empty_board.txt");
    assert!(matches!(mine_counter, Err(InputError::EmptyBoard)));
}

#[test]
fn loading_board_from_unexistent_file_returns_corresponding_error() {
    let mine_counter = Board::from_file("NO/EXISTE.txt");
    assert!(matches!(mine_counter, Err(InputError::InvalidFile(_))));
}
