use super::file_handle::*;
use napi::bindgen_prelude::*;
use rfd;

#[napi]
#[repr(transparent)]
pub struct AsyncFileDialog(pub(super) Option<rfd::AsyncFileDialog>);
#[napi]
impl AsyncFileDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        return Self(Some(rfd::AsyncFileDialog::new()));
    }

    #[napi]
    pub fn add_filter(&mut self, name: String, ext: Vec<String>) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        // https://users.rust-lang.org/t/vec-string-to-str/12619/2
        let ext: Vec<&str> = ext.iter().map(<_>::as_ref).collect();
        let x = x.add_filter(&name, &ext);
        return Ok(Self(Some(x)));
    }

    #[napi]
    pub fn set_directory(&mut self, dir: String) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_directory(&dir);
        return Ok(Self(Some(x)));
    }

    #[napi]
    pub fn set_file_name(&mut self, name: String) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_file_name(&name);
        return Ok(Self(Some(x)));
    }

    #[napi]
    pub fn set_title(&mut self, title: String) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_title(&title);
        return Ok(Self(Some(x)));
    }

    #[napi]
    pub async unsafe fn pick_file(&mut self) -> Result<Option<FileHandle>> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let handle = x.pick_file().await;
        if handle.is_none() {
            return Ok(None);
        }
        let handle = handle.unwrap();
        return Ok(Some(FileHandle(handle)));
    }

    #[napi]
    pub async unsafe fn pick_files(&mut self) -> Result<Option<Vec<FileHandle>>> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let handles = x.pick_files().await;
        if handles.is_none() {
            return Ok(None);
        }
        let handles = handles.unwrap();
        let handles = handles
            .into_iter()
            .map(|handle| FileHandle(handle))
            .collect();
        return Ok(Some(handles));
    }

    #[napi]
    pub async unsafe fn pick_folder(&mut self) -> Result<Option<FileHandle>> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let handle = x.pick_folder().await;
        if handle.is_none() {
            return Ok(None);
        }
        let handle = handle.unwrap();
        return Ok(Some(FileHandle(handle)));
    }

    #[napi]
    pub async unsafe fn pick_folders(&mut self) -> Result<Option<Vec<FileHandle>>> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let handles = x.pick_folders().await;
        if handles.is_none() {
            return Ok(None);
        }
        let handles = handles.unwrap();
        let handles = handles
            .into_iter()
            .map(|handle| FileHandle(handle))
            .collect();
        return Ok(Some(handles));
    }

    #[napi]
    pub async unsafe fn save_file(&mut self) -> Result<Option<FileHandle>> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let handle = x.save_file().await;
        if handle.is_none() {
            return Ok(None);
        }
        let handle = handle.unwrap();
        return Ok(Some(FileHandle(handle)));
    }
}
