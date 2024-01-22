pub mod datetime_utils;
pub mod kws_decrypt;
pub mod math;

pub use datetime_utils::DateTimeUtils;
pub use datetime_utils::ToDateTimeString;
pub use kws_decrypt::decrypt_data_with_kms;
pub use math::*;
use std::num::ParseFloatError;
pub trait HasId {
    fn id(&self) -> Option<u32>;
}

pub fn parse_to_f64(input: &str) -> Result<f64, ParseFloatError> {
    input.parse::<f64>()
}
