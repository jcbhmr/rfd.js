use napi::bindgen_prelude::*;
use rfd;

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
