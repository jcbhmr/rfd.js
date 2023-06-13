#[macro_use]
extern crate napi_derive;
use napi::bindgen_prelude::*;
use napi::*;
use std::path::PathBuf;

#[napi]
pub struct FileDialog {
    n: rfd::FileDialog,
}
#[napi]
impl FileDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        let n = rfd::FileDialog::new();
        return Self { n };
    }

    #[napi]
    pub fn add_filter(&mut self, name: String, ext: Vec<String>) -> &Self {
        // https://users.rust-lang.org/t/vec-string-to-str/12619/2
        let ext: Vec<&str> = ext.iter().map(<_>::as_ref).collect();
        self.n = self.n.clone().add_filter(&name, &ext);
        return self;
    }

    #[napi]
    pub fn set_directory(&mut self, dir: String) -> &Self {
        self.n = self.n.clone().set_directory(&dir);
        return self;
    }

    #[napi]
    pub fn set_file_name(&mut self, name: String) -> &Self {
        self.n = self.n.clone().set_file_name(&name);
        return self;
    }

    #[napi]
    pub fn set_title(&mut self, title: String) -> &Self {
        self.n = self.n.clone().set_title(&title);
        return self;
    }

    #[napi]
    pub fn pick_file(&mut self) -> Option<String> {
        let path_buf = self.n.clone().pick_file()?;
        return Some(path_buf.to_str()?.to_string());
    }

    #[napi]
    pub fn pick_files(&mut self) -> Option<Vec<String>> {
        let path_bufs = self.n.clone().pick_files()?;
        let mut paths = Vec::new();
        for path_buf in path_bufs {
            paths.push(path_buf.to_str()?.to_string());
        }
        return Some(paths);
    }

    #[napi]
    pub fn pick_folder(&mut self) -> Option<String> {
        let path_buf = self.n.clone().pick_folder()?;
        return Some(path_buf.to_str()?.to_string());
    }

    #[napi]
    pub fn pick_folders(&mut self) -> Option<Vec<String>> {
        let path_bufs = self.n.clone().pick_folders()?;
        let mut paths = Vec::new();
        for path_buf in path_bufs {
            paths.push(path_buf.to_str()?.to_string());
        }
        return Some(paths);
    }

    #[napi]
    pub fn save_file(&mut self) -> Option<String> {
        let path_buf = self.n.clone().save_file()?;
        return Some(path_buf.to_str()?.to_string());
    }
}

#[napi]
pub struct AsyncFileDialog {
    n: rfd::AsyncFileDialog,
}
#[napi]
impl AsyncFileDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        let n = rfd::AsyncFileDialog::new();
        return Self { n };
    }

    #[napi]
    pub fn add_filter(&mut self, name: String, ext: Vec<String>) -> &Self {
        // https://users.rust-lang.org/t/vec-string-to-str/12619/2
        let ext: Vec<&str> = ext.iter().map(<_>::as_ref).collect();
        self.n = self.n.clone().add_filter(&name, &ext);
        return self;
    }

    #[napi]
    pub fn set_directory(&mut self, dir: String) -> &Self {
        self.n = self.n.clone().set_directory(&dir);
        return self;
    }

    #[napi]
    pub fn set_file_name(&mut self, name: String) -> &Self {
        self.n = self.n.clone().set_file_name(&name);
        return self;
    }

    #[napi]
    pub fn set_title(&mut self, title: String) -> &Self {
        self.n = self.n.clone().set_title(&title);
        return self;
    }

    #[napi]
    pub async unsafe fn pick_file(&mut self) -> Option<FileHandle> {
        let n = self.n.clone();
        let handle = n.pick_file().await?;
        return Some(FileHandle { n: handle });
    }

    #[napi]
    pub async unsafe fn pick_files(&mut self) -> Option<Vec<FileHandle>> {
        let n = self.n.clone();
        let handles = n.pick_files().await?;
        return Some(
            handles
                .into_iter()
                .map(|handle| FileHandle { n: handle })
                .collect(),
        );
    }

    #[napi]
    pub async unsafe fn pick_folder(&mut self) -> Option<FileHandle> {
        let n = self.n.clone();
        let handle = n.pick_folder().await?;
        return Some(FileHandle { n: handle });
    }

    #[napi]
    pub async unsafe fn pick_folders(&mut self) -> Option<Vec<FileHandle>> {
        let n = self.n.clone();
        let handles = n.pick_folders().await?;
        return Some(
            handles
                .into_iter()
                .map(|handle| FileHandle { n: handle })
                .collect(),
        );
    }

    #[napi]
    pub async unsafe fn save_file(&mut self) -> Option<FileHandle> {
        let n = self.n.clone();
        let handle = n.save_file().await?;
        return Some(FileHandle { n: handle });
    }
}

#[napi]
pub struct FileHandle {
    n: rfd::FileHandle,
}
#[napi]
impl FileHandle {
    #[napi(factory)]
    pub fn wrap(path_buf: String) -> Self {
        let path_buf = PathBuf::from(path_buf);
        let n = rfd::FileHandle::wrap(path_buf);
        return Self { n };
    }

    #[napi]
    pub fn file_name(&self) -> String {
        return self.n.file_name();
    }

    #[napi]
    pub fn path(&self) -> Option<String> {
        let path_buf = self.n.path();
        return Some(path_buf.to_str()?.to_string());
    }

    #[napi]
    pub async unsafe fn read(&self) -> Vec<u8> {
        let bytes = self.n.read().await;
        return bytes;
    }

    #[napi]
    pub fn to_string(&self) -> Result<String> {
        let path_buf = self.n.inner();
        if let Some(str) = path_buf.to_str() {
            return Ok(str.to_string());
        } else {
            return Err(Error::new(
                Status::GenericFailure,
                "Failed to convert path_buf to str",
            ));
        }
    }
}

#[napi]
pub struct MessageDialog {
    n: rfd::MessageDialog,
}
#[napi]
impl MessageDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        let n = rfd::MessageDialog::new();
        return Self { n };
    }

    #[napi]
    pub fn set_level(&mut self, level: MessageLevel) -> &Self {
        self.n = self.n.clone().set_level(level.to_rfd_message_level());
        return self;
    }

    #[napi]
    pub fn set_title(&mut self, title: String) -> &Self {
        self.n = self.n.clone().set_title(&title);
        return self;
    }

    #[napi]
    pub fn set_description(&mut self, description: String) -> &Self {
        self.n = self.n.clone().set_description(&description);
        return self;
    }

    #[napi]
    pub fn set_buttons(&mut self, buttons: &MessageButtons) -> &Self {
        self.n = self.n.clone().set_buttons(buttons.n.clone());
        return self;
    }

    #[napi]
    pub fn show(&mut self) -> bool {
        let n = self.n.clone();
        return n.show();
    }
}

#[napi(string_enum)]
pub enum MessageLevel {
    Info,
    Warning,
    Error,
}
impl MessageLevel {
    pub fn to_rfd_message_level(&self) -> rfd::MessageLevel {
        match self {
            MessageLevel::Info => rfd::MessageLevel::Info,
            MessageLevel::Warning => rfd::MessageLevel::Warning,
            MessageLevel::Error => rfd::MessageLevel::Error,
        }
    }
}

#[napi]
pub struct MessageButtons {
    n: rfd::MessageButtons,
}
#[napi]
impl MessageButtons {
    #[napi(js_name = "Ok")]
    pub fn ok() -> Self {
        let n = rfd::MessageButtons::Ok;
        return Self { n };
    }

    #[napi(js_name = "OkCancel")]
    pub fn ok_cancel() -> Self {
        let n = rfd::MessageButtons::OkCancel;
        return Self { n };
    }

    #[napi(js_name = "YesNo")]
    pub fn yes_no() -> Self {
        let n = rfd::MessageButtons::YesNo;
        return Self { n };
    }

    #[napi(factory, js_name = "OkCustom")]
    pub fn ok_custom(x: String) -> Self {
        let n = rfd::MessageButtons::OkCustom(x);
        return Self { n };
    }

    #[napi(factory, js_name = "OkCancelCustom")]
    pub fn ok_cancel_custom(x: String, y: String) -> Self {
        let n = rfd::MessageButtons::OkCancelCustom(x, y);
        return Self { n };
    }

    #[napi]
    pub fn to_string(&self) -> String {
        let s = match self.n.clone() {
            rfd::MessageButtons::Ok => "Ok".to_string(),
            rfd::MessageButtons::OkCancel => "OkCancel".to_string(),
            rfd::MessageButtons::YesNo => "YesNo".to_string(),
            rfd::MessageButtons::OkCustom(x) => format!("OkCustom({})", x),
            rfd::MessageButtons::OkCancelCustom(x, y) => format!("OkCancelCustom({}, {})", x, y),
        };
        return s;
    }
}

#[napi]
pub struct AsyncMessageDialog {
    n: rfd::AsyncMessageDialog,
}
#[napi]
impl AsyncMessageDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        let n = rfd::AsyncMessageDialog::new();
        return Self { n };
    }

    #[napi]
    pub fn set_level(&mut self, level: MessageLevel) -> &Self {
        self.n = self.n.clone().set_level(level.to_rfd_message_level());
        return self;
    }

    #[napi]
    pub fn set_title(&mut self, title: String) -> &Self {
        self.n = self.n.clone().set_title(&title);
        return self;
    }

    #[napi]
    pub fn set_description(&mut self, description: String) -> &Self {
        self.n = self.n.clone().set_description(&description);
        return self;
    }

    #[napi]
    pub fn set_buttons(&mut self, buttons: &MessageButtons) -> &Self {
        self.n = self.n.clone().set_buttons(buttons.n.clone());
        return self;
    }

    #[napi]
    pub async unsafe fn show(&mut self) -> bool {
        let n = self.n.clone();
        return n.show().await;
    }
}
