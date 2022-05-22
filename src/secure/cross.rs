use crate::AppError;
use crate::secure::CryptBackend;

pub struct CrossCryptBackend;

impl CryptBackend for CrossCryptBackend {
    fn crypt(data: &mut [u8]) -> Result<Vec<u8>, AppError> {
        todo!()
    }

    fn decrypt(data: &mut [u8]) -> Result<Vec<u8>, AppError> {
        todo!()
    }
}