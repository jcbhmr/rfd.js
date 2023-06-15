use super::message_buttons::*;
use super::message_level::*;
use napi::bindgen_prelude::*;
use rfd;

#[napi]
#[repr(transparent)]
pub struct AsyncMessageDialog(pub(super) Option<rfd::AsyncMessageDialog>);
#[napi]
impl AsyncMessageDialog {
    #[napi(constructor)]
    pub fn new() -> Self {
        return Self(Some(rfd::AsyncMessageDialog::new()));
    }

    #[napi]
    pub fn set_level(&mut self, level: MessageLevel) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_level(level.to_rfd_t());
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
    pub fn set_description(&mut self, description: String) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_description(&description);
        return Ok(Self(Some(x)));
    }

    #[napi]
    pub fn set_buttons(&mut self, buttons: MessageButtons) -> Result<Self> {
        if self.0.is_none() {
            return Err(Error::from_reason("Already used"));
        }
        let x = self.0.take().unwrap();
        let x = x.set_buttons(buttons.to_rfd_t());
        return Ok(Self(Some(x)));
    }

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
