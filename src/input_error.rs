//! Informacion acerca de los errores de importacion del tablero

use std::fmt;

/// Errores que pueden ocurrir al importar un tablero
pub enum InputError {
    /// El tablero contiene un caracter invalido. Almacena el primer caracter invalido encontrado.
    InvalidCharacter(char),
    /// Las lineas del tablero no tienen la misma longitud.
    InvalidBoard,
    /// No se pudo leer el archivo indicado.
    InvalidFile,
    /// El archivo estaba vacio.
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
