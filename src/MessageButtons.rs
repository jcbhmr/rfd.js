use super::FileHandle::*;
use napi::bindgen_prelude::*;
use napi::*;
use rfd;
use std::path::PathBuf;

#[napi(string_enum)]
pub enum MessageButtons {
    Ok,
    OkCancel,
    YesNo,
}
impl MessageButtons {
    pub fn to_rfd_t(&self) -> rfd::MessageButtons {
        match self {
            MessageButtons::Ok => rfd::MessageButtons::Ok,
            MessageButtons::OkCancel => rfd::MessageButtons::OkCancel,
            MessageButtons::YesNo => rfd::MessageButtons::YesNo,
        }
    }
}
