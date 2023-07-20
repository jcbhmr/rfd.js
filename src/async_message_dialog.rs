use super::message_buttons::*;
use super::message_level::*;
use napi::bindgen_prelude::*;
use rfd;

/// The builder for an asynchronous message dialog popup window. Use this to
/// show alert boxes and other messages to the user.
///
/// Example:
///
/// ```js
/// const answer = await new AsyncMessageDialog()
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
pub struct AsyncMessageDialog(pub(super) Option<rfd::AsyncMessageDialog>);
#[napi]
impl AsyncMessageDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        return Self(Some(rfd::AsyncMessageDialog::new()));
    }

    /// Sets the level of the message dialog. This determines the icon that is
    /// shown in the dialog and possibly additional platform-specific defaults.
    /// Defaults to `Info`.
    ///
    /// Example:
    ///
    /// ```js
    /// const answer = await new AsyncMessageDialog()
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
    /// const answer = await new AsyncMessageDialog()
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
    /// const answer = await new AsyncMessageDialog()
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

    /// Sets the buttons of the message dialog. Defaults to `Ok`.
    ///
    /// Example:
    ///
    /// ```js
    /// const answer = await new AsyncMessageDialog()
    ///     .setButtons('Ok')
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

    /// Shows the message dialog and returns a promise that resolves to `true`
    /// if the user clicked the primary button, or `false` if the user clicked
    /// the secondary button or closed the dialog.
    ///
    /// Note that this will **consume the builder** so that it cannot be used
    /// again!
    ///
    /// Example:
    ///
    /// ```js
    /// const answer = await new AsyncMessageDialog()
    ///     .setButtons('Ok')
    ///     .show();
    /// console.log(answer);
    /// //=> true
    /// ```
    #[napi]
    pub async unsafe fn show(&mut self) -> Result<bool> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let y = x.show().await;
        return Ok(y);
    }
}
