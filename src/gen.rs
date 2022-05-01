use getrandom::getrandom;
use crate::error::AppError;

pub fn iv() -> Result<[u8; 16], AppError> {
    let mut iv = [0u8; 16];
    getrandom(&mut iv)?;
    Ok(iv)
}