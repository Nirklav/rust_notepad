use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use druid::{Data};
use crate::error::AppError;
use crate::{gen, hash};
use crate::aes::Aes;
use crate::ui::password_text::PasswordText;

const HEADER : [u8; 3] = [b'E', b'N', b'C'];

#[derive(Clone, Data, Debug, PartialEq)]
pub enum TabContent {
    Clear {
        text: String
    },
    Opened {
        text: String,
        password: String
    },
    Closed {
        password: PasswordText
    }
}

impl TabContent {
    pub fn new(password: Option<String>) -> Result<Self, AppError> {
        Ok(match password {
            None => TabContent::Clear {
                text: String::new()
            },
            Some(password) => TabContent::Opened {
                text: String::new(),
                password
            }
        })
    }

    pub fn read(file: &mut File) -> Result<Self, AppError> {
        let mut header = [0u8; 3];
        if let Ok(_) = file.read_exact(&mut header) {
            if header == HEADER {
                return Ok(TabContent::Closed {
                    password: PasswordText::new()
                })
            }
        }

        let mut text = String::new();
        file.seek(SeekFrom::Start(0))?;
        file.read_to_string(&mut text)?;
        Ok(TabContent::Clear {
            text
        })
    }

    pub fn open(&self, file: &mut File) -> Result<Self, AppError> {
        if let TabContent::Closed { password } = &self {
            let mut header = [0u8; 3];
            file.read_exact(&mut header)?;

            let mut hash = [0u8; hash::STORAGE_HASH_SIZE];
            file.read_exact(&mut hash)?;

            hash::storage_check(password.value(), &hash)?;

            let mut iv = [0u8; 16];
            file.read_exact(&mut iv)?;

            let key = hash::cypher(password.value());
            let mut aes = Aes::decryptor(&iv, &key);
            let mut text_bytes = Cursor::new(Vec::<u8>::new());
            aes.write(file, &mut text_bytes)?;

            Ok(TabContent::Opened {
                text: String::from_utf8(text_bytes.into_inner())?,
                password: password
                    .value()
                    .to_string()
            })
        } else {
            Err(AppError::internal("File is not closed"))
        }
    }

    pub fn save(&self, file: &mut File) -> Result<bool, AppError> {
        let r = match &self {
            TabContent::Clear { text } => {
                file.write(text.as_bytes())?;
                file.flush()?;
                true
            },
            TabContent::Opened { text, password } => {
                file.write(&HEADER)?;

                let hash = hash::storage_build(&password)?;
                file.write(&hash)?;

                let iv = gen::iv()?;
                file.write(&iv)?;

                let key = hash::cypher(&password);
                let mut aes = Aes::encryptor(&iv, &key);
                let mut text_bytes = Cursor::new(text.as_bytes());
                aes.write(&mut text_bytes, file)?;

                file.flush()?;
                true
            },
            TabContent::Closed { .. } => {
                /*Nothing to do*/
                false
            }
        };
        Ok(r)
    }
}