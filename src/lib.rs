pub mod datetime_utils;
pub mod kws_decrypt;
pub mod limitied_size_map;
pub mod math;

pub use datetime_utils::DateTimeUtils;
pub use datetime_utils::ToDateTimeString;
pub use kws_decrypt::decrypt_data_with_kms;
pub use limitied_size_map::DynamicLimitedSizeMap;
pub use limitied_size_map::LimitedSizeMap;
pub use limitied_size_map::ValuePair;
pub use math::*;
use serde::Serializer;
use std::num::ParseFloatError;
use std::str::FromStr;

use rust_decimal::prelude::ToPrimitive;
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

pub fn serialize_decimal_as_f64<S>(decimal: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let f64_value = decimal.to_f64().ok_or(serde::ser::Error::custom(
        "Decimal to f64 conversion failed",
    ))?;
    serializer.serialize_f64(f64_value)
}

pub trait HasId {
    fn id(&self) -> Option<u32>;
}
