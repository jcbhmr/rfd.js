use napi::bindgen_prelude::*;
use rfd;
use std::path::PathBuf;
use std::panic;
use futures::future::FutureExt;

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
    pub fn path(&self) -> Result<String> {
        let path_buf = self.0.path();
        let path = path_buf.to_str();
        if path.is_none() {
            return Err(Error::from_reason("Invalid UTF-8"));
        }
        let path = path.unwrap();
        let path = path.to_string();
        return Ok(path);
    }

    #[napi]
    pub async unsafe fn read(&self) -> Result<Vec<u8>> {
        // TODO: https://stackoverflow.com/a/54432441
        // https://stackoverflow.com/a/35559417
        let hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        // https://stackoverflow.com/a/63159560/19522682
        let vec = self.0.read().catch_unwind().await;
        panic::set_hook(hook);
        if vec.is_err() {
            return Err(Error::from_reason("Panic"));
        }
        let vec = vec.unwrap();
        return Ok(vec);
    }
}
