use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike, Utc, Weekday};
use std::{env, time::SystemTime};

pub struct DateTimeUtils {}

pub trait ToDateTimeString {
    fn to_datetime_string(self) -> String;
}

impl ToDateTimeString for i64 {
    fn to_datetime_string(self) -> String {
        let datetime = DateTime::<Utc>::from_timestamp(self, 0).expect("Invalid timestamp");
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

impl ToDateTimeString for SystemTime {
    fn to_datetime_string(self) -> String {
        let datetime: DateTime<Utc> = self.into();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

impl DateTimeUtils {
    pub fn get_current_datetime_string() -> String {
        let current_time = Utc::now().timestamp();
        let datetime = DateTime::<Utc>::from_timestamp(current_time, 0).expect("Invalid timestamp");
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn get_current_date_string() -> String {
        let current_time = Utc::now().timestamp();
        let datetime = DateTime::<Utc>::from_timestamp(current_time, 0).expect("Invalid timestamp");
        datetime.format("%Y-%m-%d").to_string()
    }
}

pub fn get_local_time() -> (i64, String) {
    let offset_seconds = env::var("TIMEZONE_OFFSET")
        .unwrap_or_else(|_| "3600".to_string())
        .parse::<i32>()
        .expect("Invalid TIMEZONE_OFFSET");

    let offset = FixedOffset::east_opt(offset_seconds).expect("Invalid offset");

    let utc_now: DateTime<Utc> = Utc::now();
    let local_now = utc_now.with_timezone(&offset);
    (
        local_now.timestamp(),
        local_now.format("%Y-%m-%dT%H:%M:%S%z").to_string(),
    )
}

pub fn is_sunday() -> bool {
    let current_date = Utc::now();
    current_date.weekday().num_days_from_sunday() == 0
}

pub fn has_remaining_sunday_hours(threshold: u32) -> bool {
    if threshold > 24 {
        panic!("Threshold must be between 0 and 24");
    }

    let now = Utc::now();

    if now.weekday() != Weekday::Sun {
        return true;
    }

    // Calculate the remaining hours in Sunday
    let remaining_seconds =
        (23 - now.hour()) * 3600 + (59 - now.minute()) * 60 + (59 - now.second());
    let remaining_hours = remaining_seconds as f64 / 3600.0;

    // Check if the remaining hours are greater than or equal to the threshold
    remaining_hours >= threshold as f64
}

pub fn num_seconds_from_midnight() -> u32 {
    let now = Local::now().time();
    now.num_seconds_from_midnight()
}
