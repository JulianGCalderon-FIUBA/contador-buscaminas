//! Informacion acerca de los errores de importacion del tablero

use std::fmt;
use std::io;

/// Errores que pueden ocurrir al importar un tablero
pub enum InputError {
    /// El tablero contiene un caracter invalido. Almacena el primer caracter invalido encontrado.
    InvalidCharacter(char),
    /// Las lineas del tablero no tienen la misma longitud.
    InvalidBoard,
    /// No se pudo leer el archivo indicado. Almacena el error obtenido en la lectura del archivo.
    InvalidFile(io::Error),
    /// El archivo esta vacio.
    EmptyBoard,
}

impl fmt::Debug for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter(invalid_character) => {
                write!(f, "Board had an invalid character: {}", invalid_character)
            }
            Self::InvalidBoard => write!(f, "Board should have all equal-sized lines"),
            Self::InvalidFile(io_error) => write!(f, "File could not be read [{}]", io_error),
            Self::EmptyBoard => write!(f, "File was empty"),
        }
    }
}

impl From<std::io::Error> for InputError {
    fn from(error: io::Error) -> Self {
        Self::InvalidFile(error)
    }
}
