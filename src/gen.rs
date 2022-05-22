use getrandom::getrandom;
use crate::error::AppError;

pub fn iv() -> Result<[u8; 16], AppError> {
    let mut iv = [0u8; 16];
    getrandom(&mut iv)?;
    Ok(iv)
}

pub fn bytes(size: usize) -> Result<Vec<u8>, AppError> {
    let mut entropy = vec![0; size];
    getrandom(&mut entropy)?;
    Ok(entropy)
}