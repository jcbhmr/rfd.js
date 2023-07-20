use napi::bindgen_prelude::*;
use rfd;

/// Used by the `MessageDialog` and `AsyncMessageDialog` as sentinels for
/// specific icons and other platform-specific defaults that appear when a
/// dialog box is opened. For instance, 'Info' might displace an "i" icon in the
/// dialog box, while 'Error' would probably be more of a red "x" icon.
#[napi(string_enum)]
pub enum MessageLevel {
    Info,
    Warning,
    Error,
}
impl MessageLevel {
    pub(super) fn to_rfd_t(&self) -> rfd::MessageLevel {
        match self {
            MessageLevel::Info => rfd::MessageLevel::Info,
            MessageLevel::Warning => rfd::MessageLevel::Warning,
            MessageLevel::Error => rfd::MessageLevel::Error,
        }
    }
}
