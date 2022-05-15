use std::io;
use std::string::FromUtf8Error;
use serde_json;
use getrandom;
use crypto::symmetriccipher::SymmetricCipherError;
use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal error: {0}")]
    Internal(&'static str),
    #[error("File with that name already exist")]
    FileAlreadyExist,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Backup file not found")]
    BackupNotFound,
    #[error("Io error: {0}")]
    Io(#[from] io::Error),
    #[error("Cypher error")]
    Cypher(SymmetricCipherError),
    #[error("Random error: {0}")]
    Random(#[from] getrandom::Error),
    #[error("FromUtf8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Zip error: {0}")]
    Zip(#[from] ZipError)
}

impl AppError {
    pub fn internal(message: &'static str) -> Self {
        AppError::Internal(message)
    }
}

impl From<SymmetricCipherError> for AppError {
    fn from(e: SymmetricCipherError) -> Self {
        AppError::Cypher(e)
    }
}