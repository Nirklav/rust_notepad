use crate::AppError;
use crate::secure::CryptBackend;

pub struct CrossCryptBackend;

impl CryptBackend for CrossCryptBackend {
    fn crypt(data: &mut [u8]) -> Result<Vec<u8>, AppError> {
        // TODO: crypt
        Ok(Vec::from(data))
    }

    fn decrypt(data: &mut [u8]) -> Result<Vec<u8>, AppError> {
        // TODO: decrypt
        Ok(Vec::from(data))
    }
}