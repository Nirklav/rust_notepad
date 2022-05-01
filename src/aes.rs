use std::io::{Read, Write};
use crypto::aes::KeySize;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::symmetriccipher::{Decryptor, Encryptor, SymmetricCipherError};
use crate::error::AppError;

pub enum Aes {
    Encryptor(Box<dyn Encryptor + 'static>),
    Decryptor(Box<dyn Decryptor + 'static>),
}

impl Aes {
    pub fn encryptor(iv: &[u8], key: &[u8]) -> Self {
        Aes::Encryptor(crypto::aes::cbc_encryptor(KeySize::KeySize256, key, iv, PkcsPadding))
    }

    pub fn decryptor(iv: &[u8], key: &[u8]) -> Self {
        Aes::Decryptor(crypto::aes::cbc_decryptor(KeySize::KeySize256, key, iv, PkcsPadding))
    }

    pub fn write<I: Read, O: Write>(&mut self, input: &mut I, output: &mut O) -> Result<(), AppError> {
        let mut in_buf = [0u8; 16];
        let mut in_buf_len;
        let mut out_buf = [0u8; 32];

        let mut eof = false;
        while !eof {
            in_buf_len = input.read(&mut in_buf)?;
            eof = in_buf_len < in_buf.len();

            let mut in_ref_buf = RefReadBuffer::new(&in_buf[0..in_buf_len]);
            let mut out_ref_buf = RefWriteBuffer::new(&mut out_buf);
            self.process(&mut in_ref_buf, &mut out_ref_buf, eof)?;

            assert_eq!(in_ref_buf.remaining(), 0);

            let r = out_ref_buf.peek_read_buffer();
            output.write(r.peek_remaining())?;
        }

        Ok(())
    }

    fn process(&mut self, input: &mut RefReadBuffer, output: &mut RefWriteBuffer, eof: bool) -> Result<BufferResult, SymmetricCipherError> {
        match self {
            Aes::Encryptor(enc) => enc.encrypt(input, output, eof),
            Aes::Decryptor(dec) => dec.decrypt(input, output, eof)
        }
    }
}