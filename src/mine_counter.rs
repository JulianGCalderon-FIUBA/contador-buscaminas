//! Contiene el method object MineCounter.

pub mod input_error;

use input_error::InputError;
use std::{char, fmt};

const MINE: char = '*';
const BLANK: char = '.';

/// Cuenta las minas adyacentes a cada casilla vacia
///  a partir de un tablero proveniente de un archivo
pub struct MineCounter {
    board: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl MineCounter {
    /// Lee un tablero de un archivo y cuenta las minas adyacentes a cada casilla vacia.
    ///
    /// El archivo debe ser valido:
    ///  - Solo puede contener el caracter _bomba_: ```*``` y el caracter _vacio_: ```.```
    ///  - Todas las filas deben contener la misma cantidad de caracteres
    pub fn from_file(file_name: &str) -> Result<Self, InputError> {
        let mut mine_counter = Self::new_from_file(file_name)?;

        mine_counter.count();

        Ok(mine_counter)
    }

    fn new_from_file(file_name: &str) -> Result<Self, InputError> {
        let file_content = std::fs::read_to_string(file_name)?;
        let board = Self::to_matrix(&file_content);
        Self::assert_is_valid_board(&board)?;

        let width = board[0].len();
        let height = board.len();

        Ok(Self {
            board,
            width,
            height,
        })
    }

    /// Imprime por pantalla el tablero modificado
    pub fn display(&self) {
        for row in self.board.iter() {
            for char in row {
                print!(" {} ", char);
            }
            println!();
        }
    }
    /// Guarda en un archivo el tablero modificado
    pub fn to_file(&self) {
        todo!()
    }

    fn count(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.board[y][x] == BLANK {
                    let adyacent_bombs = self.adyacents_mines_to(y, x);
                    if adyacent_bombs > 0 {
                        self.board[y][x] = digit_to_char(adyacent_bombs);
                    }
                }
            }
        }
    }

    fn adyacents_mines_to(&self, y: usize, x: usize) -> u8 {
        let y_range = y.max(1) - 1..=y.min(self.height - 2) + 1;
        let x_range = x.max(1) - 1..=x.min(self.width - 2) + 1;

        let mut counter = 0;

        for _y in y_range {
            for _x in x_range.clone() {
                if (_x, _y) != (x, y) && self.board[_y][_x] == MINE {
                    counter += 1;
                }
            }
        }

        counter
    }
    fn assert_is_valid_board(board: &[Vec<char>]) -> Result<(), InputError> {
        if board.is_empty() {
            return Err(InputError::EmptyBoard);
        }

        for row in board.iter() {
            for &char in row {
                if char != MINE && char != BLANK {
                    return Err(InputError::InvalidCharacter);
                }
            }
            if row.len() != board[0].len() {
                return Err(InputError::InvalidBoard);
            }
        }

        Ok(())
    }

    fn to_matrix(board: &str) -> Vec<Vec<char>> {
        let mut matrix: Vec<Vec<char>> = vec![];
        let lines = board.lines();

        for line in lines {
            let chars: Vec<char> = line.chars().collect();
            matrix.push(chars);
        }

        matrix
    }
}

impl fmt::Display for MineCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.board.iter() {
            for &char in row {
                write!(f, "{}", char)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn digit_to_char(digit: u8) -> char {
    char::from_digit(digit as u32, 10).expect("Should always be valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_matrix_returns_correcly_with_square_board() {
        let matrix = MineCounter::to_matrix(".*\n*.\n");

        assert_eq!('.', matrix[0][0]);
        assert_eq!('*', matrix[0][1]);
        assert_eq!('*', matrix[1][0]);
        assert_eq!('.', matrix[1][1]);
    }

    #[test]
    fn to_matrix_returns_correcly_with_rectangular_board() {
        let matrix = MineCounter::to_matrix(".*\n*.\n.*\n");

        assert_eq!('.', matrix[0][0]);
        assert_eq!('*', matrix[0][1]);
        assert_eq!('*', matrix[1][0]);
        assert_eq!('.', matrix[1][1]);
        assert_eq!('.', matrix[2][0]);
        assert_eq!('*', matrix[2][1]);
    }

    #[test]
    fn adyacent_mines_returns_adyacent_mines_to_position_correctly() {
        let mine_counter = MineCounter::new_from_file("boards/basic.txt").unwrap();

        assert_eq!(0, mine_counter.adyacents_mines_to(3, 0));
        assert_eq!(1, mine_counter.adyacents_mines_to(0, 0));
        assert_eq!(2, mine_counter.adyacents_mines_to(2, 1));
        assert_eq!(3, mine_counter.adyacents_mines_to(0, 2));
    }

    #[test]
    fn count_updates_board_with_adyacent_mines_count_for_empty_positons() {
        let mut mine_counter = MineCounter::new_from_file("boards/basic.txt").unwrap();
        mine_counter.count();

        assert_eq!('.', mine_counter.board[3][0]);
        assert_eq!('*', mine_counter.board[0][1]);
        assert_eq!('1', mine_counter.board[0][0]);
        assert_eq!('2', mine_counter.board[2][1]);
        assert_eq!('3', mine_counter.board[0][2]);
    }

    #[test]
    fn assert_is_valid_with_invalid_character_returns_error() {
        let board = vec![vec!['a', '.'], vec!['.', '*']];
        assert!(matches!(
            MineCounter::assert_is_valid_board(&board),
            Err(InputError::InvalidCharacter)
        ));
    }

    #[test]
    fn assert_is_valid_with_invalid_board_returns_error() {
        let board = vec![vec!['*', '.', '.'], vec!['*', '.']];
        assert!(matches!(
            MineCounter::assert_is_valid_board(&board),
            Err(InputError::InvalidBoard)
        ));
    }

    #[test]
    fn assert_is_valid_with_empty_board_returns_errror() {
        let board = vec![];
        assert!(matches!(
            MineCounter::assert_is_valid_board(&board),
            Err(InputError::EmptyBoard)
        ));
    }

    #[test]
    fn new_with_invalid_file_returns_error() {
        let board = MineCounter::new_from_file("NO/EXISTE.txt");
        assert!(matches!(board, Err(InputError::InvalidFile)));
    }
}
