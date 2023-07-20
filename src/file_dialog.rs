use napi::bindgen_prelude::*;
use rfd;

/// Synchronous file dialog builder. Use this to show file open/save dialogs.
/// It's recommended to use the `AsyncFileDialog` builder instead since it lets
/// the Node.js event loop continue even while the dialog is open.
///
/// Example:
///
/// ```js
/// // This will BLOCK the thread until the user closes the dialog.
/// const path = new FileDialog()
///     .addFilter('Text files', ['txt'])
///     .setDirectory('/home/username/Documents')
///     .setFileName('hello.txt')
///     .setTitle('Save file')
///     .saveFile();
/// console.log(path);
/// //=> '/home/username/Documents/hello.txt'
/// ```
#[napi]
#[repr(transparent)]
pub struct FileDialog(pub(super) Option<rfd::FileDialog>);
#[napi]
impl FileDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        return Self(Some(rfd::FileDialog::new()));
    }

    /// Adds a filter to the file dialog. The filter consists of a name and a
    /// list of extensions. The name is shown in the file dialog and the
    /// extensions are used to filter the files that are shown. Use this to give
    /// users a hint of what kind of file they should select.
    ///
    /// Example:
    ///
    /// ```js
    /// const path = new FileDialog()
    ///     .addFilter('Text files', ['txt'])
    ///     .pickFile();
    /// console.log(path);
    /// //=> '/home/username/Documents/hello.txt'
    /// ```
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

    /// Sets the directory that the file dialog will open in. The default is
    /// platform specific. Use this to drop the user in the most likely spot
    /// where they would find the file you want them to open.
    ///
    /// Example:
    ///
    /// ```js
    /// const path = new FileDialog()
    ///    .setDirectory('/home/username/Documents')
    ///    .pickFile();
    /// console.log(path);
    /// //=> '/home/username/Documents/hello.txt'
    #[napi]
    pub fn set_directory(&mut self, dir: String) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_directory(&dir);
        return Ok(Self(Some(x)));
    }

    /// Sets the file name that the file dialog will default to. Useful to give
    /// the user a hint of what file name to use when saving a file.
    ///
    /// Example:
    ///
    /// ```js
    /// const path = new FileDialog()
    ///     .setFileName('hello.txt')
    ///     .saveFile();
    /// console.log(path);
    /// //=> '/home/username/Documents/hello.txt'
    /// ```
    #[napi]
    pub fn set_file_name(&mut self, name: String) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_file_name(&name);
        return Ok(Self(Some(x)));
    }

    /// Sets the title of the file dialog. Defaults to an empty string. Returns
    /// `this` for chaining.
    ///
    /// Example:
    ///
    /// ```js
    /// const path = new FileDialog()
    ///     .setTitle('Save file')
    ///     .saveFile();
    /// console.log(path);
    /// //=> '/home/username/Documents/hello.txt'
    /// ```
    #[napi]
    pub fn set_title(&mut self, title: String) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_title(&title);
        return Ok(Self(Some(x)));
    }

    /// Shows the file dialog and blocks the thread until the user closes it.
    /// Returns the path of the file that the user selected or `null` if the
    /// user canceled the dialog.
    ///
    /// Example:
    ///
    /// ```js
    /// const path = new FileDialog()
    ///     .pickFile();
    /// console.log(path);
    /// //=> '/home/username/Documents/hello.txt'
    /// ```
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

    /// Shows the file dialog and blocks the thread until the user closes it.
    /// Returns the paths of the files that the user selected or `null` if the
    /// user canceled the dialog.
    ///
    /// Example:
    ///
    /// ```js
    /// const paths = new FileDialog()
    ///     .pickFiles();
    /// console.log(paths);
    /// //=> ['/home/username/Documents/hello.txt', ...]
    /// ```
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

    /// Shows the file dialog and blocks the thread until the user closes it.
    /// Returns the path of the folder that the user selected or `null` if the
    /// user canceled the dialog.
    ///
    /// Example:
    ///
    /// ```js
    /// const path = new FileDialog()
    ///     .pickFolder();
    /// console.log(path);
    /// //=> '/home/username/Documents'
    /// ```
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

    /// Shows the file dialog and blocks the thread until the user closes it.
    /// Returns the paths of the folders that the user selected or `null` if the
    /// user canceled the dialog.
    ///
    /// Example:
    ///
    /// ```js
    /// const paths = new FileDialog()
    ///     .pickFolders();
    /// console.log(paths);
    /// //=> ['/home/username/Documents', ...]
    /// ```
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

    /// Shows the file dialog and blocks the thread until the user closes it.
    /// Returns the path of the file that the user selected or `null` if the
    /// user canceled the dialog.
    ///
    /// Example:
    ///
    /// ```js
    /// const path = new FileDialog()
    ///     .saveFile();
    /// console.log(path);
    /// //=> '/home/username/Documents/hello.txt'
    /// ```
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
