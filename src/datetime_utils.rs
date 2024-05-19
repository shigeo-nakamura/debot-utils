use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use std::{env, time::SystemTime};

pub struct DateTimeUtils {}

pub trait ToDateTimeString {
    fn to_datetime_string(self) -> String;
}

impl ToDateTimeString for i64 {
    fn to_datetime_string(self) -> String {
        let naive_datetime = NaiveDateTime::from_timestamp_opt(self, 0).expect("Invalid timestamp");
        naive_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
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
        let naive_datetime =
            NaiveDateTime::from_timestamp_opt(current_time, 0).expect("Invalid timestamp");
        naive_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn get_current_date_string() -> String {
        let current_time = Utc::now().timestamp();
        let naive_datetime =
            NaiveDateTime::from_timestamp_opt(current_time, 0).expect("Invalid timestamp");
        naive_datetime.format("%Y-%m-%d").to_string()
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
