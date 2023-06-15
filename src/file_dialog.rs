use napi::bindgen_prelude::*;
use rfd;

#[napi]
#[repr(transparent)]
pub struct FileDialog(pub(super) Option<rfd::FileDialog>);
#[napi]
impl FileDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        return Self(Some(rfd::FileDialog::new()));
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
    pub fn pick_file(&mut self) -> Result<Option<String>> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let path_buf = x.pick_file();
        if path_buf.is_none() {
            return Ok(None);
        }
        let path_buf = path_buf.unwrap();
        let path = path_buf.to_str();

        if path.is_none() {
            return Err(Error::from_reason("Invalid UTF-8"));
        }
        let path = path.unwrap();
        let path = path.to_string();
        return Ok(Some(path));
    }

    #[napi]
    pub fn pick_files(&mut self) -> Result<Option<Vec<String>>> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let path_bufs = x.pick_files();
        if path_bufs.is_none() {
            return Ok(None);
        }
        let path_bufs = path_bufs.unwrap();
        let mut paths = Vec::new();
        for path_buf in path_bufs {
            let path = path_buf.to_str();
            if path.is_none() {
                return Err(Error::from_reason("Invalid UTF-8"));
            }
            let path = path.unwrap();
            let path = path.to_string();
            paths.push(path);
        }
        return Ok(Some(paths));
    }

    #[napi]
    pub fn pick_folder(&mut self) -> Result<Option<String>> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let path_buf = x.pick_folder();
        if path_buf.is_none() {
            return Ok(None);
        }
        let path_buf = path_buf.unwrap();
        let path = path_buf.to_str();
        if path.is_none() {
            return Err(Error::from_reason("Invalid UTF-8"));
        }
        let path = path.unwrap();
        let path = path.to_string();
        return Ok(Some(path));
    }

    #[napi]
    pub fn pick_folders(&mut self) -> Result<Option<Vec<String>>> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let path_bufs = x.pick_folders();
        if path_bufs.is_none() {
            return Ok(None);
        }
        let path_bufs = path_bufs.unwrap();
        let mut paths = Vec::new();
        for path_buf in path_bufs {
            let path = path_buf.to_str();
            if path.is_none() {
                return Err(Error::from_reason("Invalid UTF-8"));
            }
            let path = path.unwrap();
            let path = path.to_string();
            paths.push(path);
        }
        return Ok(Some(paths));
    }

    #[napi]
    pub fn save_file(&mut self) -> Result<Option<String>> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let path_buf = x.save_file();
        if path_buf.is_none() {
            return Ok(None);
        }
        let path_buf = path_buf.unwrap();
        let path = path_buf.to_str();
        if path.is_none() {
            return Err(Error::from_reason("Invalid UTF-8"));
        }
        let path = path.unwrap();
        let path = path.to_string();
        return Ok(Some(path));
    }
}
