use buscaminas_v2::mine_counter::input_error::InputError;
use buscaminas_v2::mine_counter::CountMines;

#[test]
fn valid_board_displays_correctly() {
    let mine_counter = CountMines::from_file("boards/basic.txt").unwrap();
    assert_eq!("1*3*1\n13*31\n.2*2.\n.111.\n", format!("{}", mine_counter))
}

#[test]
fn counting_mines_for_board_with_invalid_character_fails() {
    let mine_counter = CountMines::from_file("boards/invalid_character.txt");
    assert!(matches!(mine_counter, Err(InputError::InvalidCharacter)));
}

#[test]
fn counting_mines_for_board_with_invalid_board_fails() {
    let mine_counter = CountMines::from_file("boards/invalid_board.txt");
    assert!(matches!(mine_counter, Err(InputError::InvalidBoard)));
}

#[test]
fn counting_mines_for_empty_board_fails() {
    let mine_counter = CountMines::from_file("boards/empty_board.txt");
    assert!(matches!(mine_counter, Err(InputError::EmptyBoard)));
}

#[test]
fn counting_mines_for_invalid_file_fails() {
    let mine_counter = CountMines::from_file("NO/EXISTE.txt");
    assert!(matches!(mine_counter, Err(InputError::InvalidFile)));
}
