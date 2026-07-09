#![allow(clippy::not_unsafe_ptr_arg_deref)]
use hayashi_plugin_sdk::{hayashi_fn, hayashi_plugin};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Datelike, Timelike, Duration, Utc};

hayashi_plugin!();

// ── helpers internos ──────────────────────────────────────────────────────────

/// Converte timestamp em ms para NaiveDateTime (UTC).
/// Retorna epoch se o timestamp estiver fora do intervalo suportado pelo chrono.
#[inline]
fn ts_to_dt(ms: i64) -> NaiveDateTime {
    DateTime::from_timestamp_millis(ms)
        .map(|dt| dt.naive_utc())
        .unwrap_or_else(|| DateTime::from_timestamp_millis(0)
            .expect("epoch válido")
            .naive_utc())
}

/// Converte ano/mês/dia para NaiveDate, usando o último dia válido do mês se necessário.
#[inline]
fn ymd(year: i32, month: u32, day: u32) -> NaiveDate {
    // Tenta o dia exato, depois reduz até encontrar um válido (máx retrocede 3 dias).
    for d in (day.saturating_sub(3)..=day).rev() {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month, d) {
            return date;
        }
    }
    // Fallback seguro: primeiro dia do mês
    NaiveDate::from_ymd_opt(year, month, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).expect("epoch válido"))
}

// ── funções exportadas ────────────────────────────────────────────────────────

/// 1. parse_date(date_str, format)
/// Parse a date string into a timestamp (milliseconds since epoch).
/// Returns 0 if the string does not match the format.
/// format: "%Y-%m-%d" for "2024-01-15", "%d/%m/%Y" for "15/01/2024"
#[hayashi_fn]
pub fn parse_date(date_str: String, format: String) -> i64 {
    NaiveDate::parse_from_str(&date_str, &format)
        .ok()
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|dt| dt.and_utc().timestamp_millis())
        .unwrap_or(0)
}

/// 2. now()
/// Return current timestamp in milliseconds since epoch.
#[hayashi_fn]
pub fn now() -> i64 {
    Utc::now().timestamp_millis()
}

/// 3. add_days(timestamp, days)
/// Add (or subtract if negative) days to a timestamp.
#[hayashi_fn]
pub fn add_days(timestamp: i64, days: i64) -> i64 {
    let dt = ts_to_dt(timestamp) + Duration::days(days);
    dt.and_utc().timestamp_millis()
}

/// 4. add_months(timestamp, months)
/// Add (or subtract if negative) months to a timestamp.
/// Days are clamped to the last valid day of the resulting month.
#[hayashi_fn]
pub fn add_months(timestamp: i64, months: i64) -> i64 {
    let dt = ts_to_dt(timestamp);
    let mut year = dt.year();
    let mut month = dt.month() as i64 + months;
    while month > 12 { year += 1; month -= 12; }
    while month < 1  { year -= 1; month += 12; }
    let date = ymd(year, month as u32, dt.day());
    date.and_hms_opt(dt.hour(), dt.minute(), dt.second())
        .map(|d| d.and_utc().timestamp_millis())
        .unwrap_or(0)
}

/// 5. add_years(timestamp, years)
/// Add (or subtract if negative) years to a timestamp.
/// Days are clamped to the last valid day of the resulting month (handles Feb 29).
#[hayashi_fn]
pub fn add_years(timestamp: i64, years: i64) -> i64 {
    let dt = ts_to_dt(timestamp);
    let year = dt.year() + years as i32;
    let date = ymd(year, dt.month(), dt.day());
    date.and_hms_opt(dt.hour(), dt.minute(), dt.second())
        .map(|d| d.and_utc().timestamp_millis())
        .unwrap_or(0)
}

/// 6. diff_days(timestamp1, timestamp2)
/// Return difference in days (timestamp1 - timestamp2).
#[hayashi_fn]
pub fn diff_days(timestamp1: i64, timestamp2: i64) -> i64 {
    ts_to_dt(timestamp1)
        .signed_duration_since(ts_to_dt(timestamp2))
        .num_days()
}

/// 7. format_date(timestamp, format)
/// Format a timestamp as a date string.
/// format: "%Y-%m-%d" for "2024-01-15", "%d/%m/%Y" for "15/01/2024"
#[hayashi_fn]
pub fn format_date(timestamp: i64, format: String) -> String {
    ts_to_dt(timestamp).format(&format).to_string()
}

/// 8. year(timestamp)
/// Extract year from timestamp.
#[hayashi_fn]
pub fn year(timestamp: i64) -> i32 {
    ts_to_dt(timestamp).year()
}

/// 9. month(timestamp)
/// Extract month from timestamp (1–12).
#[hayashi_fn]
pub fn month(timestamp: i64) -> i64 {
    ts_to_dt(timestamp).month() as i64
}

/// 10. day(timestamp)
/// Extract day-of-month from timestamp (1–31).
#[hayashi_fn]
pub fn day(timestamp: i64) -> i64 {
    ts_to_dt(timestamp).day() as i64
}

/// 11. hour(timestamp)
/// Extract hour from timestamp (0–23).
#[hayashi_fn]
pub fn hour(timestamp: i64) -> i64 {
    ts_to_dt(timestamp).hour() as i64
}

/// 12. minute(timestamp)
/// Extract minute from timestamp (0–59).
#[hayashi_fn]
pub fn minute(timestamp: i64) -> i64 {
    ts_to_dt(timestamp).minute() as i64
}

/// 13. second(timestamp)
/// Extract second from timestamp (0–59).
#[hayashi_fn]
pub fn second(timestamp: i64) -> i64 {
    ts_to_dt(timestamp).second() as i64
}

/// 14. weekday(timestamp)
/// Extract weekday from timestamp (0=Sunday, 1=Monday, …, 6=Saturday).
#[hayashi_fn]
pub fn weekday(timestamp: i64) -> i64 {
    ts_to_dt(timestamp).weekday().num_days_from_sunday() as i64
}

/// 15. is_weekend(timestamp)
/// Return true if the timestamp falls on Saturday or Sunday.
#[hayashi_fn]
pub fn is_weekend(timestamp: i64) -> bool {
    let w = ts_to_dt(timestamp).weekday().num_days_from_sunday();
    w == 0 || w == 6
}

/// 16. start_of_month(timestamp)
/// Return timestamp for the first day of the month at 00:00:00 UTC.
#[hayashi_fn]
pub fn start_of_month(timestamp: i64) -> i64 {
    let dt = ts_to_dt(timestamp);
    NaiveDate::from_ymd_opt(dt.year(), dt.month(), 1)
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|d| d.and_utc().timestamp_millis())
        .unwrap_or(0)
}

/// 17. end_of_month(timestamp)
/// Return timestamp for the last day of the month at 23:59:59 UTC.
#[hayashi_fn]
pub fn end_of_month(timestamp: i64) -> i64 {
    let dt = ts_to_dt(timestamp);
    let (next_year, next_month) = if dt.month() == 12 {
        (dt.year() + 1, 1u32)
    } else {
        (dt.year(), dt.month() + 1)
    };
    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .and_then(|d| d.pred_opt())
        .and_then(|d| d.and_hms_opt(23, 59, 59))
        .map(|d| d.and_utc().timestamp_millis())
        .unwrap_or(0)
}

/// 18. start_of_year(timestamp)
/// Return timestamp for January 1 of the year at 00:00:00 UTC.
#[hayashi_fn]
pub fn start_of_year(timestamp: i64) -> i64 {
    let dt = ts_to_dt(timestamp);
    NaiveDate::from_ymd_opt(dt.year(), 1, 1)
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .map(|d| d.and_utc().timestamp_millis())
        .unwrap_or(0)
}

/// 19. end_of_year(timestamp)
/// Return timestamp for December 31 of the year at 23:59:59 UTC.
#[hayashi_fn]
pub fn end_of_year(timestamp: i64) -> i64 {
    let dt = ts_to_dt(timestamp);
    NaiveDate::from_ymd_opt(dt.year(), 12, 31)
        .and_then(|d| d.and_hms_opt(23, 59, 59))
        .map(|d| d.and_utc().timestamp_millis())
        .unwrap_or(0)
}

/// 20. days_in_month(timestamp)
/// Return the number of days in the month of the timestamp.
#[hayashi_fn]
pub fn days_in_month(timestamp: i64) -> i64 {
    let dt = ts_to_dt(timestamp);
    let (next_year, next_month) = if dt.month() == 12 {
        (dt.year() + 1, 1u32)
    } else {
        (dt.year(), dt.month() + 1)
    };
    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .and_then(|d| d.pred_opt())
        .map(|d| d.day() as i64)
        .unwrap_or(30)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ancoragem: 2024-03-15 12:30:45 UTC
    // 2024-03-15 00:00:00 UTC = 1710460800000 ms
    // + 12h30m45s = 45045000 ms → total = 1710505845000 ms
    const T: i64 = 1_710_505_845_000;

    // ── parse_date / format_date (round-trip) ─────────────────────────────────

    #[test]
    fn test_parse_date_iso() {
        let ts = __hayashi_impl_parse_date("2024-03-15".to_string(), "%Y-%m-%d".to_string());
        // Deve retornar o timestamp de 2024-03-15 00:00:00 UTC
        assert_eq!(ts, 1_710_460_800_000);
    }

    #[test]
    fn test_parse_date_br_format() {
        let ts = __hayashi_impl_parse_date("15/03/2024".to_string(), "%d/%m/%Y".to_string());
        assert_eq!(ts, 1_710_460_800_000);
    }

    #[test]
    fn test_parse_date_invalid() {
        let ts = __hayashi_impl_parse_date("not-a-date".to_string(), "%Y-%m-%d".to_string());
        assert_eq!(ts, 0);
    }

    #[test]
    fn test_format_date_roundtrip() {
        let ts = __hayashi_impl_parse_date("2024-03-15".to_string(), "%Y-%m-%d".to_string());
        let s  = __hayashi_impl_format_date(ts, "%Y-%m-%d".to_string());
        assert_eq!(s, "2024-03-15");
    }

    // ── extratores de componentes ─────────────────────────────────────────────

    #[test]
    fn test_year() {
        assert_eq!(__hayashi_impl_year(T), 2024);
    }

    #[test]
    fn test_month() {
        assert_eq!(__hayashi_impl_month(T), 3);
    }

    #[test]
    fn test_day() {
        assert_eq!(__hayashi_impl_day(T), 15);
    }

    #[test]
    fn test_hour() {
        assert_eq!(__hayashi_impl_hour(T), 12);
    }

    #[test]
    fn test_minute() {
        assert_eq!(__hayashi_impl_minute(T), 30);
    }

    #[test]
    fn test_second() {
        assert_eq!(__hayashi_impl_second(T), 45);
    }

    // ── weekday / is_weekend ──────────────────────────────────────────────────

    #[test]
    fn test_weekday_friday() {
        // 2024-03-15 é sexta-feira; num_days_from_sunday(Friday) = 5
        assert_eq!(__hayashi_impl_weekday(T), 5);
    }

    #[test]
    fn test_is_weekend_false() {
        assert!(!__hayashi_impl_is_weekend(T)); // sexta-feira
    }

    #[test]
    fn test_is_weekend_true() {
        // 2024-03-16 é sábado: T + 86400000 ms
        assert!(__hayashi_impl_is_weekend(T + 86_400_000));
    }

    // ── add_days / diff_days ──────────────────────────────────────────────────

    #[test]
    fn test_add_days_positive() {
        let ts = __hayashi_impl_parse_date("2024-01-01".to_string(), "%Y-%m-%d".to_string());
        let ts2 = __hayashi_impl_add_days(ts, 30);
        assert_eq!(__hayashi_impl_format_date(ts2, "%Y-%m-%d".to_string()), "2024-01-31");
    }

    #[test]
    fn test_add_days_negative() {
        let ts = __hayashi_impl_parse_date("2024-02-01".to_string(), "%Y-%m-%d".to_string());
        let ts2 = __hayashi_impl_add_days(ts, -1);
        assert_eq!(__hayashi_impl_format_date(ts2, "%Y-%m-%d".to_string()), "2024-01-31");
    }

    #[test]
    fn test_diff_days() {
        let t1 = __hayashi_impl_parse_date("2024-12-31".to_string(), "%Y-%m-%d".to_string());
        let t2 = __hayashi_impl_parse_date("2024-01-01".to_string(), "%Y-%m-%d".to_string());
        assert_eq!(__hayashi_impl_diff_days(t1, t2), 365); // 2024 é bissexto
    }

    // ── add_months / add_years ────────────────────────────────────────────────

    #[test]
    fn test_add_months() {
        let ts = __hayashi_impl_parse_date("2024-01-15".to_string(), "%Y-%m-%d".to_string());
        let ts2 = __hayashi_impl_add_months(ts, 3);
        assert_eq!(__hayashi_impl_format_date(ts2, "%Y-%m".to_string()), "2024-04");
    }

    #[test]
    fn test_add_months_year_wrap() {
        let ts = __hayashi_impl_parse_date("2024-11-15".to_string(), "%Y-%m-%d".to_string());
        let ts2 = __hayashi_impl_add_months(ts, 3);
        assert_eq!(__hayashi_impl_format_date(ts2, "%Y-%m".to_string()), "2025-02");
    }

    #[test]
    fn test_add_years() {
        let ts = __hayashi_impl_parse_date("2020-06-15".to_string(), "%Y-%m-%d".to_string());
        let ts2 = __hayashi_impl_add_years(ts, 4);
        assert_eq!(__hayashi_impl_format_date(ts2, "%Y".to_string()), "2024");
    }

    // ── start/end of month/year ───────────────────────────────────────────────

    #[test]
    fn test_start_of_month() {
        let ts  = __hayashi_impl_parse_date("2024-03-15".to_string(), "%Y-%m-%d".to_string());
        let som = __hayashi_impl_start_of_month(ts);
        assert_eq!(__hayashi_impl_format_date(som, "%Y-%m-%d".to_string()), "2024-03-01");
    }

    #[test]
    fn test_end_of_month_march() {
        let ts  = __hayashi_impl_parse_date("2024-03-15".to_string(), "%Y-%m-%d".to_string());
        let eom = __hayashi_impl_end_of_month(ts);
        assert_eq!(__hayashi_impl_format_date(eom, "%Y-%m-%d".to_string()), "2024-03-31");
    }

    #[test]
    fn test_end_of_month_february_leap() {
        let ts  = __hayashi_impl_parse_date("2024-02-10".to_string(), "%Y-%m-%d".to_string());
        let eom = __hayashi_impl_end_of_month(ts);
        assert_eq!(__hayashi_impl_format_date(eom, "%Y-%m-%d".to_string()), "2024-02-29");
    }

    #[test]
    fn test_start_of_year() {
        let ts  = __hayashi_impl_parse_date("2024-07-04".to_string(), "%Y-%m-%d".to_string());
        let soy = __hayashi_impl_start_of_year(ts);
        assert_eq!(__hayashi_impl_format_date(soy, "%Y-%m-%d".to_string()), "2024-01-01");
    }

    #[test]
    fn test_end_of_year() {
        let ts  = __hayashi_impl_parse_date("2024-07-04".to_string(), "%Y-%m-%d".to_string());
        let eoy = __hayashi_impl_end_of_year(ts);
        assert_eq!(__hayashi_impl_format_date(eoy, "%Y-%m-%d".to_string()), "2024-12-31");
    }

    // ── days_in_month ─────────────────────────────────────────────────────────

    #[test]
    fn test_days_in_month_march() {
        let ts = __hayashi_impl_parse_date("2024-03-01".to_string(), "%Y-%m-%d".to_string());
        assert_eq!(__hayashi_impl_days_in_month(ts), 31);
    }

    #[test]
    fn test_days_in_month_feb_leap() {
        let ts = __hayashi_impl_parse_date("2024-02-01".to_string(), "%Y-%m-%d".to_string());
        assert_eq!(__hayashi_impl_days_in_month(ts), 29);
    }

    #[test]
    fn test_days_in_month_feb_nonleap() {
        let ts = __hayashi_impl_parse_date("2023-02-01".to_string(), "%Y-%m-%d".to_string());
        assert_eq!(__hayashi_impl_days_in_month(ts), 28);
    }
}
