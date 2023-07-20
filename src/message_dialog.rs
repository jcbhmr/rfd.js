use super::message_buttons::*;
use super::message_level::*;
use napi::bindgen_prelude::*;
use rfd;

/// The builder for a synchronous message dialog popup window. Use this to show
/// alert boxes and other messages to the user. It's recommended to use the async
/// `AsyncMessageDialog` builder instead since it lets the Node.js event loop
/// continue even while the dialog is open.
///
/// Example:
///
/// ```js
/// const answer = new MessageDialog()
///     .setLevel('Error')
///     .setTitle('Uh oh!')
///     .setDescription('Something went wrong.')
///     .setButtons('Ok')
///     .show();
/// console.log(answer);
/// //=> true
/// ```
#[napi]
#[repr(transparent)]
pub struct MessageDialog(pub(super) Option<rfd::MessageDialog>);
#[napi]
impl MessageDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        return Self(Some(rfd::MessageDialog::new()));
    }

    /// Sets the level of the message dialog. This determines the icon that is
    /// shown in the dialog and possibly additional platform-specific defaults.
    /// Defaults to `Info`.
    ///
    /// Example:
    ///
    /// ```js
    /// const answer = new MessageDialog()
    ///     .setLevel('Error')
    ///     .show();
    /// ```
    #[napi]
    pub fn set_level(&mut self, level: MessageLevel) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_level(level.to_rfd_t());
        return Ok(Self(Some(x)));
    }

    /// Sets the title of the message dialog. Defaults to an empty string.
    /// Returns `this` for chaining.
    ///
    /// Example:
    ///
    /// ```js
    /// const answer = new MessageDialog()
    ///     .setTitle('Uh oh!')
    ///     .show();
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

    /// Sets the description of the message dialog. Defaults to an empty string.
    /// Returns `this` for chaining.
    ///
    /// Example:
    ///
    /// ```js
    /// const answer = new MessageDialog()
    ///     .setDescription('Something went wrong.')
    ///     .show();
    /// ```
    #[napi]
    pub fn set_description(&mut self, description: String) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_description(&description);
        return Ok(Self(Some(x)));
    }

    /// Sets the buttons that are shown in the message dialog. Defaults to `Ok`.
    /// Returns `this` for chaining.
    ///
    /// Example:
    ///
    /// ```js
    /// const answer = new MessageDialog()
    ///     .setButtons('OkCancel')
    ///     .show();
    /// ```
    #[napi]
    pub fn set_buttons(&mut self, buttons: MessageButtons) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_buttons(buttons.to_rfd_t());
        return Ok(Self(Some(x)));
    }

    /// Shows the message dialog. Returns `true` if the user clicked the
    /// affirmative button, `false` otherwise.
    ///
    /// Example:
    ///
    /// ```js
    /// const answer = new MessageDialog()
    ///     .show();
    /// console.log(answer);
    /// //=> true
    /// ```
    #[napi]
    pub fn show(&mut self) -> Result<bool> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let y = x.show();
        return Ok(y);
    }
}
