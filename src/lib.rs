use hayashi_plugin_sdk::{hayashi_fn, hayashi_plugin};
use chrono::{NaiveDate, NaiveDateTime, Datelike, Timelike, Duration, Utc};

hayashi_plugin!();

/// 1. parse_date(date_str, format)
/// Parse a date string into a timestamp (milliseconds since epoch)
/// format: "%Y-%m-%d" for "2024-01-15", "%d/%m/%Y" for "15/01/2024"
#[hayashi_fn]
pub fn parse_date(date_str: String, format: String) -> i64 {
    match NaiveDate::parse_from_str(&date_str, &format) {
        Ok(date) => {
            let datetime = date.and_hms_opt(0, 0, 0).unwrap();
            datetime.and_utc().timestamp_millis()
        }
        Err(_) => {
            eprintln!("Failed to parse date: {} with format: {}", date_str, format);
            0
        }
    }
}

/// 2. now()
/// Return current timestamp in milliseconds since epoch
#[hayashi_fn]
pub fn now() -> i64 {
    Utc::now().timestamp_millis()
}

/// 3. add_days(timestamp, days)
/// Add days to a timestamp
#[hayashi_fn]
pub fn add_days(timestamp: i64, days: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    let new_datetime = datetime + Duration::days(days);
    new_datetime.and_utc().timestamp_millis()
}

/// 4. add_months(timestamp, months)
/// Add months to a timestamp
#[hayashi_fn]
pub fn add_months(timestamp: i64, months: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    let mut year = datetime.year();
    let mut month = datetime.month() as i64 + months;
    
    while month > 12 {
        year += 1;
        month -= 12;
    }
    while month < 1 {
        year -= 1;
        month += 12;
    }
    
    let day = datetime.day().min(28); // Use 28 to avoid invalid dates
    let new_datetime = NaiveDate::from_ymd_opt(year, month as u32, day)
        .unwrap()
        .and_hms_opt(datetime.hour(), datetime.minute(), datetime.second())
        .unwrap();
    new_datetime.and_utc().timestamp_millis()
}

/// 5. add_years(timestamp, years)
/// Add years to a timestamp
#[hayashi_fn]
pub fn add_years(timestamp: i64, years: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    let year = datetime.year() + years as i32;
    let month = datetime.month();
    let day = datetime.day().min(28); // Use 28 to avoid invalid dates
    let new_datetime = NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(datetime.hour(), datetime.minute(), datetime.second())
        .unwrap();
    new_datetime.and_utc().timestamp_millis()
}

/// 6. diff_days(timestamp1, timestamp2)
/// Return difference in days between two timestamps
#[hayashi_fn]
pub fn diff_days(timestamp1: i64, timestamp2: i64) -> i64 {
    let dt1 = NaiveDateTime::from_timestamp_millis(timestamp1).unwrap();
    let dt2 = NaiveDateTime::from_timestamp_millis(timestamp2).unwrap();
    let duration = dt1.signed_duration_since(dt2);
    duration.num_days()
}

/// 7. format_date(timestamp, format)
/// Format a timestamp as a date string
/// format: "%Y-%m-%d" for "2024-01-15", "%d/%m/%Y" for "15/01/2024"
#[hayashi_fn]
pub fn format_date(timestamp: i64, format: String) -> String {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    datetime.format(&format).to_string()
}

/// 8. year(timestamp)
/// Extract year from timestamp
#[hayashi_fn]
pub fn year(timestamp: i64) -> i32 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    datetime.year()
}

/// 9. month(timestamp)
/// Extract month from timestamp (1-12)
#[hayashi_fn]
pub fn month(timestamp: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    datetime.month() as i64
}

/// 10. day(timestamp)
/// Extract day from timestamp (1-31)
#[hayashi_fn]
pub fn day(timestamp: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    datetime.day() as i64
}

/// 11. hour(timestamp)
/// Extract hour from timestamp (0-23)
#[hayashi_fn]
pub fn hour(timestamp: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    datetime.hour() as i64
}

/// 12. minute(timestamp)
/// Extract minute from timestamp (0-59)
#[hayashi_fn]
pub fn minute(timestamp: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    datetime.minute() as i64
}

/// 13. second(timestamp)
/// Extract second from timestamp (0-59)
#[hayashi_fn]
pub fn second(timestamp: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    datetime.second() as i64
}

/// 14. weekday(timestamp)
/// Extract weekday from timestamp (0=Sunday, 1=Monday, ..., 6=Saturday)
#[hayashi_fn]
pub fn weekday(timestamp: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    datetime.weekday().num_days_from_sunday() as i64
}

/// 15. is_weekend(timestamp)
/// Check if timestamp falls on weekend (Saturday or Sunday)
#[hayashi_fn]
pub fn is_weekend(timestamp: i64) -> bool {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    let weekday = datetime.weekday().num_days_from_sunday();
    weekday == 0 || weekday == 6
}

/// 16. start_of_month(timestamp)
/// Return timestamp for start of month (first day at 00:00:00)
#[hayashi_fn]
pub fn start_of_month(timestamp: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    let year = datetime.year();
    let month = datetime.month();
    let new_datetime = NaiveDate::from_ymd_opt(year, month, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    new_datetime.and_utc().timestamp_millis()
}

/// 17. end_of_month(timestamp)
/// Return timestamp for end of month (last day at 23:59:59)
#[hayashi_fn]
pub fn end_of_month(timestamp: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    let year = datetime.year();
    let month = datetime.month();
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };
    let last_day = NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
        .day();
    let new_datetime = NaiveDate::from_ymd_opt(year, month, last_day)
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();
    new_datetime.and_utc().timestamp_millis()
}

/// 18. start_of_year(timestamp)
/// Return timestamp for start of year (January 1 at 00:00:00)
#[hayashi_fn]
pub fn start_of_year(timestamp: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    let year = datetime.year();
    let new_datetime = NaiveDate::from_ymd_opt(year, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    new_datetime.and_utc().timestamp_millis()
}

/// 19. end_of_year(timestamp)
/// Return timestamp for end of year (December 31 at 23:59:59)
#[hayashi_fn]
pub fn end_of_year(timestamp: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    let year = datetime.year();
    let new_datetime = NaiveDate::from_ymd_opt(year, 12, 31)
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();
    new_datetime.and_utc().timestamp_millis()
}

/// 20. days_in_month(timestamp)
/// Return number of days in the month of the timestamp
#[hayashi_fn]
pub fn days_in_month(timestamp: i64) -> i64 {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp).unwrap();
    let year = datetime.year();
    let month = datetime.month();
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };
    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
        .day() as i64
}
