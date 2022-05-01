use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::string::FromUtf8Error;
use getrandom;
use crypto::symmetriccipher::SymmetricCipherError;

#[derive(fmt::Debug)]
pub enum AppError {
    Internal(&'static str),
    FileAlreadyExist,
    InvalidPassword,
    Io(io::Error),
    Cypher(SymmetricCipherError),
    Random(getrandom::Error),
    FromUtf8(FromUtf8Error)
}

impl AppError {
    pub fn internal(message: &'static str) -> Self {
        AppError::Internal(message)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Internal(s) => s.fmt(f),
            AppError::Io(e) => e.fmt(f),
            AppError::FileAlreadyExist => write!(f, "File with that name already exist"),
            AppError::InvalidPassword => write!(f, "Invalid password"),
            AppError::Cypher(e) => fmt::Debug::fmt(&e, f),
            AppError::Random(e) => e.fmt(f),
            AppError::FromUtf8(e) => e.fmt(f),
        }
    }
}

impl Error for AppError {

}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<SymmetricCipherError> for AppError {
    fn from(e: SymmetricCipherError) -> Self {
        AppError::Cypher(e)
    }
}

impl From<&'static str> for AppError {
    fn from(text: &'static str) -> Self {
        AppError::Internal(text)
    }
}

impl From<getrandom::Error> for AppError {
    fn from(e: getrandom::Error) -> Self {
        AppError::Random(e)
    }
}

impl From<FromUtf8Error> for AppError {
    fn from(e: FromUtf8Error) -> Self {
        AppError::FromUtf8(e)
    }
}