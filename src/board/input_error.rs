//! Este modulo contiene informacion acerca de los errores de lectura del tablero

use std::fmt;

/// Errores que pueden ocurrir al leer un tablero
///
/// Las variaciones del InputError son:
///  - InvalidCharacter: El tablero contiene un caracter invalido. Contiene el caracter invalido.
///  - InvalidBoard: Las lineas del tablero no tienen la misma longitud.
///  - InvalidFile: No se pudo leer el archivo indicado.
///  - EmptyBoard: El archivo estaba vacio.
pub enum InputError {
    InvalidCharacter(char),
    InvalidBoard,
    InvalidFile,
    EmptyBoard,
}

impl fmt::Debug for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter(invalid_character) => {
                write!(f, "Board had an invalid character: {}", invalid_character)
            }
            Self::InvalidBoard => write!(f, "Board should have all equal-sized lines"),
            Self::InvalidFile => write!(f, "File could not be read"),
            Self::EmptyBoard => write!(f, "File was empty"),
        }
    }
}

impl From<std::io::Error> for InputError {
    fn from(_: std::io::Error) -> Self {
        Self::InvalidFile
    }
}
