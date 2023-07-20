use napi::bindgen_prelude::*;
use rfd;

/// Used by the `MessageDialog` and `AsyncMessageDialog` as sentinels for
/// specific button configurations.
#[napi(string_enum)]
pub enum MessageButtons {
    Ok,
    OkCancel,
    YesNo,
}
impl MessageButtons {
    pub(super) fn to_rfd_t(&self) -> rfd::MessageButtons {
        match self {
            MessageButtons::Ok => rfd::MessageButtons::Ok,
            MessageButtons::OkCancel => rfd::MessageButtons::OkCancel,
            MessageButtons::YesNo => rfd::MessageButtons::YesNo,
        }
    }
}
