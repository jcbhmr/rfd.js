use super::file_handle::*;
use napi::bindgen_prelude::*;
use rfd;

/// An asynchronous file dialog builder. Use this to open a file dialog as a
/// Promise instance. The `.pickFile()` and other methods will return a
/// `Promise` and invalidate the builder object. You can only use the builder to
/// perform a single `.pickFile()` or similar operation.
///
/// Unlike the synchronous version, this doesn't return the path string directly
/// but instead wraps it in a `FileHandle` instance (not the Node.js `node:fs`
/// `FileHandle` class).
///
/// Example:
///
/// ```js
/// const fileHandle = await new AsyncFileDialog()
///   .addFilter('Images', ['png', 'jpg', 'jpeg'])
///   .setDirectory('~/Pictures')
///   .pickFile();
/// console.log(fileHandle.path());
/// //=> '/home/username/Pictures/image.png'
/// ```
#[napi]
#[repr(transparent)]
pub struct AsyncFileDialog(pub(super) Option<rfd::AsyncFileDialog>);
#[napi]
impl AsyncFileDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        return Self(Some(rfd::AsyncFileDialog::new()));
    }

    /// Add a file filter. The first argument is the name of the filter, the
    /// which will be used on supported platforms as the friendly name for the
    /// list of extensions. The second argument is a list of extensions, which
    /// can be used to limit the viewable files in the file dialog. This is
    /// useful when you want to select only specific supported extensions or
    /// file types.
    ///
    /// Example:
    ///
    /// ```js
    /// const fileHandle = await new AsyncFileDialog()
    ///    .addFilter('JavaScript', ['js', 'mjs', 'cjs'])
    ///    .pickFile();
    /// console.log(fileHandle.path());
    /// //=> '/home/username/Documents/script.js'
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

    /// Set the directory that the file dialog will open in. By default this is
    /// unset and will open in whatever default configuration the OS provides.
    /// This is useful to set when you want to open the file dialog in a
    /// specific directory like the `~/Downloads` directory or `~/Pictures`.
    /// This **does not** prevent the user from navigating to other directories
    /// outside of the starting directory.
    ///
    /// Example:
    ///
    /// ```js
    /// const fileHandle = await new AsyncFileDialog()
    ///     .setDirectory('~/Pictures')
    ///     .pickFile();
    /// console.log(fileHandle.path());
    /// //=> '/home/username/Pictures/image.jpeg'
    /// ```
    #[napi]
    pub fn set_directory(&mut self, dir: String) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_directory(&dir);
        return Ok(Self(Some(x)));
    }

    /// Set the default file name that will be pre-filled in the file dialog
    /// for the "Save as..."-type dialogs. This is useful when you want to
    /// recommend a particular file extension like `Document.pdf` or similar.
    /// This **does not** prevent the user from changing the file name or
    /// completely changing the extension!
    ///
    /// Example:
    ///
    /// ```js
    /// const fileHandle = await new AsyncFileDialog()
    ///     .setFileName('Document.txt')
    ///     .saveFile();
    /// console.log(fileHandle.path());
    /// //=> '/home/username/Downloads/My_Document.txt'
    /// await writeFile(fileHandle.path(), 'Hello, world!');
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

    /// Set the title of the file dialog. That's the thing that appears on the
    /// top bar of the window. Do this to override the default "Save as..." or
    /// whatever platform-specific default title is used. This doesn't affect
    /// anything else besides the UI element.
    ///
    /// Example:
    ///
    /// ```js
    /// new AsyncFileDialog().setTitle('Save your document');
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

    /// Actually pick the file and consume the builder. This will return a
    /// `Promise` that resolves to a `FileHandle` instance. The `FileHandle`
    /// instance can be used to get the path of the file that was picked.
    ///
    /// Note that this **will invalidate the builder**. The recommended pattern
    /// is to construct and consume the builder in one continuous chain like
    /// this example:
    ///
    /// ```js
    /// const fileHandle = await new AsyncFileDialog()
    ///   .addFilter('Images', ['png', 'jpg', 'jpeg'])
    ///   .setDirectory('~/Pictures')
    ///   .pickFile();
    /// console.log(fileHandle.path());
    /// //=> '/home/username/Pictures/image.png'
    /// ```
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

    /// Same as `.pickFile()` but allows the user to select multiple files.
    /// These will be returned as a list of `FileHandle` instances.
    ///
    /// Example:
    ///
    /// ```js
    /// const fileHandles = await new AsyncFileDialog()
    ///  .addFilter('Images', ['png', 'jpg', 'jpeg'])
    ///  .setDirectory('~/Pictures')
    ///  .pickFiles();
    /// console.log(fileHandles.map(handle => handle.path()));
    /// //=> ['/home/username/Pictures/image.png', ...]
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

    /// Same as `.pickFile()` but allows the user to select a folder. This will
    /// return a `FileHandle` instance. Note that this `FileHandle` instance
    /// will not be `.read()`-able since it's a folder, not a file.
    ///
    /// Example:
    ///
    /// ```js
    /// const fileHandle = await new AsyncFileDialog()
    ///  .setDirectory('~/Pictures')
    ///  .pickFolder();
    /// console.log(fileHandle.path());
    /// //=> '/home/username/Pictures'
    /// ```
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

    /// Same as `.pickFolder()` but allows the user to select multiple folders.
    /// These will be returned as a list of `FileHandle` instances. Note that
    /// these `FileHandle` instances will not be `.read()`-able since they're
    /// folders, not files.
    ///
    /// Example:
    ///
    /// ```js
    /// const fileHandles = await new AsyncFileDialog()
    ///     .setDirectory('~/Pictures')
    ///     .pickFolders();
    /// console.log(fileHandles.map(handle => handle.path()));
    /// //=> ['/home/username/Pictures', ...]
    /// ```
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

    /// Same as `.pickFile()` but allows the user to select a file to save to.
    /// This file doesn't have to exist on-disk like when the user picks a file
    /// using `.pickFile()`. This will return a `FileHandle` instance.
    /// This `FileHandle` instance can be used to get the path of the file that
    /// was picked, but may not be `.read()`-able since it may not exist yet.
    ///
    /// Example:
    ///
    /// ```js
    /// const fileHandle = await new AsyncFileDialog()
    ///     .setFileName('Document.txt')
    ///     .saveFile();
    /// console.log(fileHandle.path());
    /// //=> '/home/username/Downloads/My_Document.txt'
    /// await writeFile(fileHandle.path(), 'Hello, world!');
    /// ```
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
