#[macro_use]
extern crate napi_derive;

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
      let pathbuf = self.n.clone().pick_file()?;
      return Some(pathbuf.to_str()?.to_string());
    }

    #[napi]
    pub fn pick_files(&mut self) -> Option<Vec<String>> {
      let pathbufs = self.n.clone().pick_files()?;
      let mut paths = Vec::new();
      for pathbuf in pathbufs {
        paths.push(pathbuf.to_str()?.to_string());
      }
      return Some(paths);
    }

    #[napi]
    pub fn pick_folder(&mut self) -> Option<String> {
      let pathbuf = self.n.clone().pick_folder()?;
      return Some(pathbuf.to_str()?.to_string());
    }

    #[napi]
    pub fn pick_folders(&mut self) -> Option<Vec<String>> {
      let pathbufs = self.n.clone().pick_folders()?;
      let mut paths = Vec::new();
      for pathbuf in pathbufs {
        paths.push(pathbuf.to_str()?.to_string());
      }
      return Some(paths);
    }

    #[napi]
    pub fn save_file(&mut self) -> Option<String> {
      let pathbuf = self.n.clone().save_file()?;
      return Some(pathbuf.to_str()?.to_string());
    }
}
