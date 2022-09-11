//! Logica para la importacion/exportacion del tablero y el conteo de minas

use crate::input_error::InputError;
use std::{char, fmt, fs, io};

const MINE: char = '*';
const BLANK: char = '.';

/// Almacena el tablero
pub struct Board {
    board: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

/// Importacion/Exportacion del tablero
impl Board {
    /// Lee un tablero de un archivo y lo carga en un struct ```Board```
    ///
    /// El archivo debe ser valido:
    ///  - Solo puede contener el caracter _bomba_: ```*``` y el caracter _vacio_: ```.```
    ///  - Todas las filas deben contener la misma cantidad de caracteres
    pub fn from_file(file_name: &str) -> Result<Self, InputError> {
        let file_content = fs::read_to_string(file_name)?;
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

    /// Imprime el tablero a la salida estandar.
    pub fn display(&self) {
        print!("{}", self);
    }

    /// Guarda el tablero en un archivo. Devuelve error de IO en caso de fallar.
    pub fn to_file(&self, file_name: &str) -> io::Result<()> {
        fs::write(file_name, self.to_string())
    }

    /// Verifica que el tablero sea valido y devuelve un error correspondiente en caso contrario.
    fn assert_is_valid_board(board: &[Vec<char>]) -> Result<(), InputError> {
        if board.is_empty() {
            return Err(InputError::EmptyBoard);
        }

        for row in board.iter() {
            for &char in row {
                if char != MINE && char != BLANK {
                    return Err(InputError::InvalidCharacter(char));
                }
            }
            if row.len() != board[0].len() {
                return Err(InputError::InvalidBoard);
            }
        }

        Ok(())
    }

    /// Convierte un String en una matriz de caracteres, separando por lineas.
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

/// Conteo de minas
impl Board {
    /// Cuenta las minas adyacentes a cada celda vacia y actualiza el tablero
    pub fn count_mines(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.board[y][x] == BLANK {
                    let adyacent_bombs = self.adyacents_mines_to(y, x);
                    if adyacent_bombs > 0 {
                        self.board[y][x] = Self::digit_to_char(adyacent_bombs);
                    }
                }
            }
        }
    }

    /// Devuelve la cantidad de bombas adyacentes a la posicion indicada
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

    fn digit_to_char(digit: u8) -> char {
        char::from_digit(digit as u32, 10).expect("Always valid")
    }
}

impl fmt::Display for Board {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_matrix_returns_correcly_with_square_board() {
        let matrix = Board::to_matrix(".*\n*.\n");

        assert_eq!('.', matrix[0][0]);
        assert_eq!('*', matrix[0][1]);
        assert_eq!('*', matrix[1][0]);
        assert_eq!('.', matrix[1][1]);
    }

    #[test]
    fn to_matrix_returns_correcly_with_rectangular_board() {
        let matrix = Board::to_matrix(".*\n*.\n.*\n");

        assert_eq!('.', matrix[0][0]);
        assert_eq!('*', matrix[0][1]);
        assert_eq!('*', matrix[1][0]);
        assert_eq!('.', matrix[1][1]);
        assert_eq!('.', matrix[2][0]);
        assert_eq!('*', matrix[2][1]);
    }

    #[test]
    fn adyacent_mines_returns_adyacent_mines_to_position_correctly() {
        let mine_counter = Board::from_file("boards/basic.txt").unwrap();

        assert_eq!(0, mine_counter.adyacents_mines_to(3, 0));
        assert_eq!(1, mine_counter.adyacents_mines_to(0, 0));
        assert_eq!(2, mine_counter.adyacents_mines_to(2, 1));
        assert_eq!(3, mine_counter.adyacents_mines_to(0, 2));
    }

    #[test]
    fn count_updates_board_with_adyacent_mines_count_for_empty_positons() {
        let mut mine_counter = Board::from_file("boards/basic.txt").unwrap();
        mine_counter.count_mines();

        assert_eq!('.', mine_counter.board[3][0]);
        assert_eq!('*', mine_counter.board[0][1]);
        assert_eq!('1', mine_counter.board[0][0]);
        assert_eq!('2', mine_counter.board[2][1]);
        assert_eq!('3', mine_counter.board[0][2]);
    }

    #[test]
    fn assert_is_valid_with_invalid_character_returns_corresponding_error() {
        let board = vec![vec!['a', '.'], vec!['.', '*']];
        assert!(matches!(
            Board::assert_is_valid_board(&board),
            Err(InputError::InvalidCharacter('a'))
        ));
    }

    #[test]
    fn assert_is_valid_with_invalid_board_returns_corresponding_error() {
        let board = vec![vec!['*', '.', '.'], vec!['*', '.']];
        assert!(matches!(
            Board::assert_is_valid_board(&board),
            Err(InputError::InvalidBoard)
        ));
    }

    #[test]
    fn assert_is_valid_with_empty_board_returns_corresponding_error() {
        let board = vec![];
        assert!(matches!(
            Board::assert_is_valid_board(&board),
            Err(InputError::EmptyBoard)
        ));
    }

    #[test]
    fn new_with_invalid_file_returns_corresponding_error() {
        let board = Board::from_file("NO/EXISTE.txt");
        assert!(matches!(board, Err(InputError::InvalidFile)));
    }
}
