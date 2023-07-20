use napi::bindgen_prelude::*;
use rfd;
use std::path::PathBuf;
use std::panic;
use futures::future::FutureExt;


/// This is not recommended for use. The
/// `FileHandle` class is only provided to achieve feature-parity with the
/// original `rfd` crate. It is recommended to use the `node:fs` or
/// `node:fs/promises` modules instead.
///
/// Example:
///
/// ```js
/// const path = '/home/username/Documents/hello.txt';
/// const fileHandle = FileHandle.wrap(path);
/// console.log(fileHandle.path());
/// //=> '/home/username/Documents/hello.txt'
/// ```
///
/// Instead, just use the native `node:fs` module:
///
/// ```js
/// const path = '/home/username/Documents/hello.txt';
/// const fileName = basename(path);
/// const bytes = await readFile(path);
/// const text = await readFile(path, 'utf8');
/// // If you really need a HANDLE, use this:
/// const fileHandle = await open(path);
/// ```
#[napi]
#[repr(transparent)]
pub struct FileHandle(pub(crate) rfd::FileHandle);
#[napi]
impl FileHandle {
    /// Creates a new `FileHandle` from a path. This is not recommended. It's
    /// here only for feature-parity with the original `rfd` crate. Instead,
    /// use `node:fs/promises` `open()` to get a file handle, or `readFile()`
    /// if you just want to read a file.
    ///
    /// Example:
    ///
    /// ```js
    /// const path = '/home/username/Documents/hello.txt';
    /// const fileHandle = FileHandle.wrap(path);
    /// console.log(fileHandle.path());
    /// //=> '/home/username/Documents/hello.txt'
    /// ```
    #[napi(factory)]
    pub fn wrap(path_buf: String) -> Self {
        let path_buf = PathBuf::from(path_buf);
        return Self(rfd::FileHandle::wrap(path_buf));
    }

    /// Returns the file name of the file that this `FileHandle` refers to. This
    /// is not guarenteed to be the same as `basename(fileHandle.path())` since
    /// it's from the Rust standard library, not Node.js' `node:fs` module. It's
    /// recommended to use the Node.js `basename()` instead to guarentee
    /// consistency across all your code.
    ///
    /// Example:
    ///
    /// ```js
    /// const fileHandle = await new AsyncFileDialog().pickFile();
    /// const fileName = fileHandle.fileName();
    /// console.log(fileName);
    /// //=> 'hello.txt'
    /// ```
    ///
    /// The better way:
    ///
    /// ```js
    /// const fileHandle = await new AsyncFileDialog().pickFile();
    /// const fileName = basename(fileHandle.path());
    /// console.log(fileName);
    /// //=> 'hello.txt'
    /// ```
    #[napi]
    pub fn file_name(&self) -> String {
        return self.0.file_name();
    }

    /// Returns the path that this `FileHandle` refers to. This is the best way
    /// to extract the underlying value from this `FileHandle`.
    ///
    /// Example:
    ///
    /// ```js
    /// const fileHandle = await new AsyncFileDialog().pickFile();
    /// const path = fileHandle.path();
    /// console.log(path);
    /// //=> '/home/username/Documents/hello.txt'
    /// ```
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

    /// Reads the entire contents of the file into a vector of bytes. This is
    /// **not** the best way to read the file. It is provided only for
    /// feature-parity with the original `rfd` crate.
    ///
    /// The recommended way to read any resulting files is via the `node:fs` or
    /// `node:fs/promises` modules.
    ///
    /// Example:
    ///
    /// ```js
    /// const fileHandle = await new AsyncFileDialog().pickFile();
    /// const bytes = await fileHandle.read();
    /// const text = new TextDecoder().decode(bytes);
    /// console.log(text);
    /// //=> 'Hello, world!'
    /// ```
    ///
    /// The better way:
    ///
    /// ```js
    /// const fileHandle = await new AsyncFileDialog().pickFile();
    /// const text = await readFile(fileHandle.path(), 'utf8');
    /// console.log(text);
    /// //=> 'Hello, world!'
    /// ```
    ///
    /// Note that if when `.read()` is called, the wrapped path that this
    /// `FileHandle` refers to happens to be a folder, not a file, this method
    /// will throw an error. This is different from `rfd` which currently
    /// panics if the path is a folder.
    ///
    /// https://github.com/PolyMeilex/rfd/issues/125
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
