pub mod datetime_utils;
pub mod kws_decrypt;
pub mod math;

pub use datetime_utils::DateTimeUtils;
pub use datetime_utils::ToDateTimeString;
pub use kws_decrypt::decrypt_data_with_kms;
pub use math::*;
use std::num::ParseFloatError;
use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal::Error as DecimalParseError;

#[derive(Debug)]
pub enum ParseDecimalError {
    FloatError(ParseFloatError),
    DecimalError(DecimalParseError),
}

impl From<ParseFloatError> for ParseDecimalError {
    fn from(err: ParseFloatError) -> Self {
        ParseDecimalError::FloatError(err)
    }
}

impl From<DecimalParseError> for ParseDecimalError {
    fn from(err: DecimalParseError) -> Self {
        ParseDecimalError::DecimalError(err)
    }
}

pub fn parse_to_decimal(input: &str) -> Result<Decimal, ParseDecimalError> {
    Decimal::from_str(input).map_err(Into::into)
}

pub fn parse_to_f64(input: &str) -> Result<f64, ParseFloatError> {
    input.parse::<f64>()
}

pub trait HasId {
    fn id(&self) -> Option<u32>;
}
