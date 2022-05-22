pub mod windows;
pub mod cross;

use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{DeserializeOwned, Visitor};
use crate::AppError;
use crate::secure::windows::WindowsCryptBackend;

macro_rules! tri {
    ($i:ident, $e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(serde::$i::Error::custom(err)),
        }
    };
}

#[cfg(windows)]
pub type Secure<V> = _Secure<WindowsCryptBackend, V>;

#[cfg(not(windows))]
pub type Secure<V> = _Secure<CrossCryptBackend, V>;

#[derive(Debug)]
pub struct _Secure<B, V>
    where
        B: CryptBackend + Debug,
        V: Serialize + DeserializeOwned + Debug
{
    backend: PhantomData<B>,
    value: V,
}

impl<B, V> _Secure<B, V>
    where
        B: CryptBackend + Debug,
        V: Serialize + DeserializeOwned + Debug
{
    pub fn new(value: V) -> Self {
        _Secure {
            backend: PhantomData::default(),
            value
        }
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
}

impl<B, V> Serialize for _Secure<B, V>
    where
        B: CryptBackend + Debug,
        V: Serialize + DeserializeOwned + Debug
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        let mut json = tri!(ser, serde_json::to_string(&self.value));
        let encrypted = tri!(ser, B::crypt(unsafe { json.as_bytes_mut() }));
        let base64 = base64::encode(encrypted);
        serializer.serialize_str(&base64)
    }
}

impl<'de, B, V> Deserialize<'de> for _Secure<B, V>
    where
        B: CryptBackend + Debug,
        V: Serialize + DeserializeOwned + Debug
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        let base64 = deserializer.deserialize_string(StringVisitor)?;
        let mut bytes = tri!(de, base64::decode(base64));
        let decrypted = tri!(de, B::decrypt(&mut bytes));
        let value = tri!(de, serde_json::from_slice::<V>(&decrypted));
        Ok(_Secure::new(value))
    }
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<String, E>
        where
            E: de::Error,
    {
        Ok(s.to_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<String, E>
        where
            E: de::Error
    {
        Ok(v)
    }
}

pub trait CryptBackend {
    fn crypt(data: &mut [u8]) -> Result<Vec<u8>, AppError>;
    fn decrypt(data: &mut [u8]) -> Result<Vec<u8>, AppError>;
}