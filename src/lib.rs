pub mod datetime_utils;
pub mod kws_decrypt;
pub mod math;

pub use datetime_utils::DateTimeUtils;
pub use datetime_utils::ToDateTimeString;
pub use kws_decrypt::decrypt_data_with_kms;
pub use math::*;

pub trait HasId {
    fn id(&self) -> Option<u32>;
}
