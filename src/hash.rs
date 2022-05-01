use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::sha3::Sha3;
use crate::error::AppError;

pub const STORAGE_HASH_SIZE : usize = 48;

const PRE_SALT : &str = "Bk8UdO46oAUe+nY1Dt5hMzazudCqegmrXta0/caJxQUjy6QcWU8FNbdAynFE+Qbyy6CYEY1sFfeTThN9IjU/u";
const POST_SALT : &str = "+QYlwnvqv+O9oBfmpJNK2wYf7MsWEFvnJUh/aW75DbNJHpNO59D9b+NDVfU7uiP05oJzDsGVUhi5ig1YdDqjz";

pub fn cypher(password: &str) -> [u8; 32] {
    let mut hasher = Sha3::sha3_256();

    hasher.input_str(PRE_SALT);
    hasher.input_str(password);
    hasher.input_str(POST_SALT);

    let mut hex = [0u8; 32];
    hasher.result(&mut hex);
    hex
}

pub fn storage_build(password: &str) -> Result<[u8; 48], AppError> {
    let mut salt = [0u8; 16];
    getrandom::getrandom(&mut salt)?;

    let mut dk = [0u8; 32];
    let mut mac = Hmac::new(Sha3::sha3_256(), password.as_bytes());

    crypto::pbkdf2::pbkdf2(&mut mac, &salt[..], 100, &mut dk);

    let mut result = [0u8; STORAGE_HASH_SIZE];
    for i in 0..16 {
        result[i] = salt[i];
    }
    for i in 0..32 {
        result[i + salt.len()] = dk[i];
    }
    Ok(result)
}

pub fn storage_check(password: &str, hash_bytes: &[u8]) -> Result<(), AppError> {
    let salt = &hash_bytes[..16];
    let input_dk = &hash_bytes[16..];

    let mut mac = Hmac::new(Sha3::sha3_256(), password.as_bytes());
    let mut dk = [0u8; 32];
    crypto::pbkdf2::pbkdf2(&mut mac, &salt[..], 100, &mut dk);

    if crypto::util::fixed_time_eq(input_dk, &dk) {
        Ok(())
    } else {
        Err(AppError::InvalidPassword)
    }
}