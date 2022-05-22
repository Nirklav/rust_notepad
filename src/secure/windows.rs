use std::io::Write;
use std::ptr;
use winapi::um::wincrypt::DATA_BLOB;
use crate::{AppError, gen};
use crate::secure::CryptBackend;

const ENTROPY_SIZE : usize = 64;

#[derive(Debug)]
pub struct WindowsCryptBackend;

#[cfg(windows)]
impl CryptBackend for WindowsCryptBackend {
    fn crypt(data: &mut [u8]) -> Result<Vec<u8>, AppError> {
        let mut entropy_bytes = gen::bytes(ENTROPY_SIZE)?;
        let mut entropy_blob = DATA_BLOB {
            pbData: entropy_bytes.as_mut_ptr(),
            cbData: entropy_bytes.len() as _
        };
        let mut in_blob = DATA_BLOB {
            pbData: data.as_mut_ptr(),
            cbData: data.len() as _
        };

        let mut out_blob = DATA_BLOB::default();
        let success = unsafe {
            winapi::um::dpapi::CryptProtectData(
                &mut in_blob as *mut _,
                ptr::null(),
                &mut entropy_blob as *mut _,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut out_blob as *mut _
            ) != 0
        };

        if success {
            let mut r = Vec::new();
            r.write(&entropy_bytes)?;

            let slice = unsafe { std::slice::from_raw_parts(out_blob.pbData, out_blob.cbData as _) };
            r.write(slice)?;

            unsafe {
                winapi::um::winbase::LocalFree(out_blob.pbData as _);
            }

            Ok(r)
        } else {
            Err(AppError::internal("Error on encrypt"))
        }
    }

    fn decrypt(data: &mut [u8]) -> Result<Vec<u8>, AppError> {
        let (entropy_bytes, data_bytes) = data.split_at_mut(ENTROPY_SIZE);
        let mut entropy_blob = DATA_BLOB {
            pbData: entropy_bytes.as_mut_ptr(),
            cbData: entropy_bytes.len() as _
        };
        let mut in_blob = DATA_BLOB {
            pbData: data_bytes.as_mut_ptr(),
            cbData: data_bytes.len() as _
        };

        let mut out_blob = DATA_BLOB::default();
        let mut out_desc = ptr::null_mut();
        let success = unsafe {
            winapi::um::dpapi::CryptUnprotectData(
                &mut in_blob as *mut _,
                &mut out_desc as *mut _,
                &mut entropy_blob as *mut _,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &mut out_blob as *mut _
            ) != 0
        };

        if success {
            let mut r = Vec::new();
            let slice = unsafe { std::slice::from_raw_parts(out_blob.pbData, out_blob.cbData as _) };
            r.write(slice)?;

            unsafe {
                winapi::um::winbase::LocalFree(out_blob.pbData as _);
                winapi::um::winbase::LocalFree(out_desc as _);
            }

            return Ok(r);
        } else {
            Err(AppError::internal("Error on encrypt"))
        }
    }
}