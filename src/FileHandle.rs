use napi::bindgen_prelude::*;
use napi::*;
use rfd;
use std::path::PathBuf;

#[napi]
#[repr(transparent)]
pub struct FileHandle(pub(crate) rfd::FileHandle);
#[napi]
impl FileHandle {
    #[napi(factory)]
    pub fn wrap(path_buf: String) -> Self {
        let path_buf = PathBuf::from(path_buf);
        return Self(rfd::FileHandle::wrap(path_buf));
    }

    #[napi]
    pub fn file_name(&self) -> String {
        return self.0.file_name();
    }

    #[napi]
    pub fn path(&self) -> Option<String> {
        let path_buf = self.0.path();
        let path = path_buf.to_str();
        if path.is_none() {
            // The path is not valid UTF-8. Could error here instead. 🤷‍♂️
            return None;
        }
        let path = path.unwrap();
        let path = path.to_string();
        return Some(path);
    }

    #[napi]
    pub async unsafe fn read(&self) -> Vec<u8> {
        return self.0.read().await;
    }

    #[napi]
    pub fn to_string(&self) -> Result<String> {
        let path_buf = self.0.inner();
        let path = path_buf.to_str();
        if path.is_none() {
            return Err(Error::from_reason("Invalid UTF-8"));
        }
        let path = path.unwrap();
        let path = path.to_string();
        return Ok(path);
    }
}
