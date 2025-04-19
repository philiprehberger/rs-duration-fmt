//! Human-readable duration formatting and parsing.
//!
//! This crate provides functions to format [`std::time::Duration`] values into
//! human-readable strings and parse such strings back into durations.
//!
//! # Supported units
//!
//! Days, hours, minutes, seconds, and milliseconds. Years, months, and weeks are
//! intentionally excluded because [`std::time::Duration`] represents a fixed span
//! of time and cannot accurately model calendar-relative units (months vary in
//! length, years have leap seconds, etc.).
//!
//! # Examples
//!
//! ```
//! use philiprehberger_duration_fmt::{format_duration, parse_duration, format_duration_iso8601, parse_iso8601_duration, format_duration_short};
//! use std::time::Duration;
//!
//! let d = Duration::from_secs(9015);
//! assert_eq!(format_duration(d), "2h 30m 15s");
//!
//! let parsed = parse_duration("2h 30m 15s").unwrap();
//! assert_eq!(parsed, d);
//!
//! // ISO 8601
//! assert_eq!(format_duration_iso8601(d), "PT2H30M15S");
//! let iso_parsed = parse_iso8601_duration("PT2H30M15S").unwrap();
//! assert_eq!(iso_parsed, d);
//!
//! // Short format
//! assert_eq!(format_duration_short(d), "2h30m15s");
//! ```

use std::fmt;
use std::time::Duration;

/// Error returned when parsing a duration string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The input string was empty or contained only whitespace.
    EmptyInput,
    /// The input string could not be recognized as a valid duration format.
    InvalidFormat(String),
    /// The parsed duration would overflow `std::time::Duration`.
    Overflow,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "empty input"),
            ParseError::InvalidFormat(msg) => write!(f, "invalid format: {msg}"),
            ParseError::Overflow => write!(f, "duration overflow"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Duration broken down into individual unit components.
struct Components {
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
    millis: u64,
}

fn decompose(d: Duration) -> Components {
    let total_millis = d.as_millis() as u64;
    let millis = total_millis % 1000;
    let total_secs = total_millis / 1000;
    let seconds = total_secs % 60;
    let total_mins = total_secs / 60;
    let minutes = total_mins % 60;
    let total_hours = total_mins / 60;
    let hours = total_hours % 24;
    let days = total_hours / 24;

    Components {
        days,
        hours,
        minutes,
        seconds,
        millis,
    }
}

/// Format a duration in compact style, skipping zero units.
///
/// Produces strings like `"2h 30m 15s"`, `"500ms"`, or `"1d 5h"`.
/// A zero duration returns `"0s"`.
///
/// # Examples
///
/// ```
/// use philiprehberger_duration_fmt::format_duration;
/// use std::time::Duration;
///
/// assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
/// assert_eq!(format_duration(Duration::ZERO), "0s");
/// ```
#[must_use]
pub fn format_duration(d: Duration) -> String {
    let c = decompose(d);
    let mut parts = Vec::new();

    if c.days > 0 {
        parts.push(format!("{}d", c.days));
    }
    if c.hours > 0 {
        parts.push(format!("{}h", c.hours));
    }
    if c.minutes > 0 {
        parts.push(format!("{}m", c.minutes));
    }
    if c.seconds > 0 {
        parts.push(format!("{}s", c.seconds));
    }
    if c.millis > 0 {
        parts.push(format!("{}ms", c.millis));
    }

    if parts.is_empty() {
        "0s".to_string()
    } else {
        parts.join(" ")
    }
}

/// Format a duration in verbose style with full unit names.
///
/// Produces strings like `"2 hours, 30 minutes, 15 seconds"`.
/// A zero duration returns `"0 seconds"`.
///
/// # Examples
///
/// ```
/// use philiprehberger_duration_fmt::format_duration_verbose;
/// use std::time::Duration;
///
/// assert_eq!(
///     format_duration_verbose(Duration::from_secs(9015)),
///     "2 hours, 30 minutes, 15 seconds"
/// );
/// ```
#[must_use]
pub fn format_duration_verbose(d: Duration) -> String {
    let c = decompose(d);
    let mut parts = Vec::new();

    fn unit_str(value: u64, singular: &str, plural: &str) -> Option<String> {
        match value {
            0 => None,
            1 => Some(format!("1 {singular}")),
            n => Some(format!("{n} {plural}")),
        }
    }

    if let Some(s) = unit_str(c.days, "day", "days") {
        parts.push(s);
    }
    if let Some(s) = unit_str(c.hours, "hour", "hours") {
        parts.push(s);
    }
    if let Some(s) = unit_str(c.minutes, "minute", "minutes") {
        parts.push(s);
    }
    if let Some(s) = unit_str(c.seconds, "second", "seconds") {
        parts.push(s);
    }
    if let Some(s) = unit_str(c.millis, "millisecond", "milliseconds") {
        parts.push(s);
    }

    if parts.is_empty() {
        "0 seconds".to_string()
    } else {
        parts.join(", ")
    }
}

/// Format a duration in compact style, limited to the top `max_units` units.
///
/// This is useful when you want a shorter representation. For example,
/// `max_units=2` on a duration of 1 day, 1 hour, 1 minute, and 1 second
/// would produce `"1d 1h"`.
///
/// # Examples
///
/// ```
/// use philiprehberger_duration_fmt::format_duration_precise;
/// use std::time::Duration;
///
/// let d = Duration::from_secs(90061);
/// assert_eq!(format_duration_precise(d, 2), "1d 1h");
/// assert_eq!(format_duration_precise(d, 1), "1d");
/// ```
#[must_use]
pub fn format_duration_precise(d: Duration, max_units: usize) -> String {
    let c = decompose(d);
    let mut parts = Vec::new();

    if c.days > 0 {
        parts.push(format!("{}d", c.days));
    }
    if c.hours > 0 {
        parts.push(format!("{}h", c.hours));
    }
    if c.minutes > 0 {
        parts.push(format!("{}m", c.minutes));
    }
    if c.seconds > 0 {
        parts.push(format!("{}s", c.seconds));
    }
    if c.millis > 0 {
        parts.push(format!("{}ms", c.millis));
    }

    if parts.is_empty() {
        "0s".to_string()
    } else {
        parts.truncate(max_units);
        parts.join(" ")
    }
}

/// Multiplier in seconds for each recognized compact/verbose unit suffix.
fn unit_to_millis(unit: &str) -> Result<u64, ParseError> {
    match unit {
        "ms" | "millisecond" | "milliseconds" => Ok(1),
        "s" | "sec" | "second" | "seconds" => Ok(1_000),
        "m" | "min" | "minute" | "minutes" => Ok(60_000),
        "h" | "hour" | "hours" => Ok(3_600_000),
        "d" | "day" | "days" => Ok(86_400_000),
        other => Err(ParseError::InvalidFormat(format!(
            "unknown unit: '{other}'"
        ))),
    }
}

/// Parse a compact duration string into a [`Duration`].
///
/// Accepted formats include `"2h30m"`, `"2h 30m 15s"`, `"500ms"`, and `"0s"`.
/// Whitespace between components is optional.
///
/// # Errors
///
/// Returns [`ParseError::EmptyInput`] if the string is empty, or
/// [`ParseError::InvalidFormat`] if it cannot be parsed.
///
/// # Examples
///
/// ```
/// use philiprehberger_duration_fmt::parse_duration;
/// use std::time::Duration;
///
/// assert_eq!(parse_duration("2h30m").unwrap(), Duration::from_secs(9000));
/// assert_eq!(parse_duration("500ms").unwrap(), Duration::from_millis(500));
/// ```
pub fn parse_duration(s: &str) -> Result<Duration, ParseError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let mut total_millis: u64 = 0;
    let mut chars = s.as_bytes();
    let mut found_any = false;

    while !chars.is_empty() {
        // Skip whitespace
        while chars.first().is_some_and(|c| c.is_ascii_whitespace()) {
            chars = &chars[1..];
        }
        if chars.is_empty() {
            break;
        }

        // Parse number
        let num_start = chars;
        let mut num_len = 0;
        while num_len < chars.len() && chars[num_len].is_ascii_digit() {
            num_len += 1;
        }
        if num_len == 0 {
            return Err(ParseError::InvalidFormat(format!(
                "expected a number near '{}'",
                String::from_utf8_lossy(chars)
            )));
        }
        let num_str = std::str::from_utf8(&num_start[..num_len]).unwrap();
        let value: u64 = num_str
            .parse()
            .map_err(|_| ParseError::Overflow)?;
        chars = &chars[num_len..];

        // Skip optional whitespace between number and unit
        while chars.first().is_some_and(|c| c.is_ascii_whitespace()) {
            chars = &chars[1..];
        }

        // Parse unit
        let mut unit_len = 0;
        while unit_len < chars.len() && chars[unit_len].is_ascii_alphabetic() {
            unit_len += 1;
        }
        if unit_len == 0 {
            return Err(ParseError::InvalidFormat(
                "expected a unit suffix after number".to_string(),
            ));
        }
        let unit = std::str::from_utf8(&chars[..unit_len]).unwrap();
        let multiplier = unit_to_millis(unit)?;
        chars = &chars[unit_len..];

        total_millis = total_millis
            .checked_add(value.checked_mul(multiplier).ok_or(ParseError::Overflow)?)
            .ok_or(ParseError::Overflow)?;
        found_any = true;
    }

    if !found_any {
        return Err(ParseError::EmptyInput);
    }

    Ok(Duration::from_millis(total_millis))
}

/// Parse a verbose duration string into a [`Duration`].
///
/// Accepted formats include `"2 hours 30 minutes"`, `"1 day, 5 hours"`, and
/// `"500 milliseconds"`. Commas between components are optional.
///
/// # Errors
///
/// Returns [`ParseError::EmptyInput`] if the string is empty, or
/// [`ParseError::InvalidFormat`] if it cannot be parsed.
///
/// # Examples
///
/// ```
/// use philiprehberger_duration_fmt::parse_duration_verbose;
/// use std::time::Duration;
///
/// assert_eq!(
///     parse_duration_verbose("2 hours, 30 minutes").unwrap(),
///     Duration::from_secs(9000)
/// );
/// ```
pub fn parse_duration_verbose(s: &str) -> Result<Duration, ParseError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    // Remove commas and extra whitespace, then parse like compact
    let cleaned: String = s
        .replace(',', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    // The verbose format has spaces between numbers and units, which
    // parse_duration already handles. We can delegate directly.
    parse_duration(&cleaned)
}

/// Format a duration as an ISO 8601 duration string.
///
/// Produces strings like `"PT2H30M15S"`. Zero components are omitted.
/// A zero duration returns `"PT0S"`.
///
/// # Examples
///
/// ```
/// use philiprehberger_duration_fmt::format_duration_iso8601;
/// use std::time::Duration;
///
/// assert_eq!(format_duration_iso8601(Duration::from_secs(9015)), "PT2H30M15S");
/// assert_eq!(format_duration_iso8601(Duration::ZERO), "PT0S");
/// ```
#[must_use]
pub fn format_duration_iso8601(d: Duration) -> String {
    let total_secs = d.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours == 0 && minutes == 0 && seconds == 0 {
        return "PT0S".to_string();
    }

    let mut result = "PT".to_string();
    if hours > 0 {
        result.push_str(&format!("{hours}H"));
    }
    if minutes > 0 {
        result.push_str(&format!("{minutes}M"));
    }
    if seconds > 0 {
        result.push_str(&format!("{seconds}S"));
    }
    result
}

/// Parse an ISO 8601 duration string into a [`Duration`].
///
/// Accepts strings starting with `PT` followed by optional hours (`H`),
/// minutes (`M`), and seconds (`S`) components. For example: `"PT2H30M15S"`,
/// `"PT5M"`, `"PT0S"`.
///
/// # Errors
///
/// Returns [`ParseError::EmptyInput`] if the string is empty, or
/// [`ParseError::InvalidFormat`] if it does not start with `PT` or contains
/// unrecognized components.
///
/// # Examples
///
/// ```
/// use philiprehberger_duration_fmt::parse_iso8601_duration;
/// use std::time::Duration;
///
/// assert_eq!(parse_iso8601_duration("PT2H30M15S").unwrap(), Duration::from_secs(9015));
/// assert_eq!(parse_iso8601_duration("PT5M").unwrap(), Duration::from_secs(300));
/// ```
pub fn parse_iso8601_duration(s: &str) -> Result<Duration, ParseError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    if !s.starts_with("PT") {
        return Err(ParseError::InvalidFormat(
            "ISO 8601 duration must start with 'PT'".to_string(),
        ));
    }

    let body = &s[2..];
    if body.is_empty() {
        return Err(ParseError::InvalidFormat(
            "no components after 'PT'".to_string(),
        ));
    }

    let mut total_secs: u64 = 0;
    let mut chars = body.as_bytes();
    let mut found_any = false;

    while !chars.is_empty() {
        // Parse number
        let mut num_len = 0;
        while num_len < chars.len() && chars[num_len].is_ascii_digit() {
            num_len += 1;
        }
        if num_len == 0 {
            return Err(ParseError::InvalidFormat(format!(
                "expected a number near '{}'",
                String::from_utf8_lossy(chars)
            )));
        }
        let num_str = std::str::from_utf8(&chars[..num_len]).unwrap();
        let value: u64 = num_str.parse().map_err(|_| ParseError::Overflow)?;
        chars = &chars[num_len..];

        if chars.is_empty() {
            return Err(ParseError::InvalidFormat(
                "expected H, M, or S after number".to_string(),
            ));
        }

        let unit = chars[0];
        chars = &chars[1..];

        let multiplier = match unit {
            b'H' => 3600,
            b'M' => 60,
            b'S' => 1,
            _ => {
                return Err(ParseError::InvalidFormat(format!(
                    "unexpected unit '{}', expected H, M, or S",
                    unit as char
                )));
            }
        };

        total_secs = total_secs
            .checked_add(value.checked_mul(multiplier).ok_or(ParseError::Overflow)?)
            .ok_or(ParseError::Overflow)?;
        found_any = true;
    }

    if !found_any {
        return Err(ParseError::InvalidFormat(
            "no components found".to_string(),
        ));
    }

    Ok(Duration::from_secs(total_secs))
}

/// Format a duration in short abbreviated style without spaces.
///
/// Produces strings like `"2h30m15s"`. Zero components are omitted.
/// A zero duration returns `"0s"`.
///
/// # Examples
///
/// ```
/// use philiprehberger_duration_fmt::format_duration_short;
/// use std::time::Duration;
///
/// assert_eq!(format_duration_short(Duration::from_secs(9015)), "2h30m15s");
/// assert_eq!(format_duration_short(Duration::ZERO), "0s");
/// ```
#[must_use]
pub fn format_duration_short(d: Duration) -> String {
    let c = decompose(d);
    let mut result = String::new();

    if c.days > 0 {
        result.push_str(&format!("{}d", c.days));
    }
    if c.hours > 0 {
        result.push_str(&format!("{}h", c.hours));
    }
    if c.minutes > 0 {
        result.push_str(&format!("{}m", c.minutes));
    }
    if c.seconds > 0 {
        result.push_str(&format!("{}s", c.seconds));
    }
    if c.millis > 0 {
        result.push_str(&format!("{}ms", c.millis));
    }

    if result.is_empty() {
        "0s".to_string()
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- format_duration ----

    #[test]
    fn format_zero() {
        assert_eq!(format_duration(Duration::ZERO), "0s");
    }

    #[test]
    fn format_seconds_only() {
        assert_eq!(format_duration(Duration::from_secs(45)), "45s");
    }

    #[test]
    fn format_minutes_and_seconds() {
        assert_eq!(format_duration(Duration::from_secs(125)), "2m 5s");
    }

    #[test]
    fn format_hours_minutes_seconds() {
        assert_eq!(format_duration(Duration::from_secs(9015)), "2h 30m 15s");
    }

    #[test]
    fn format_days() {
        assert_eq!(format_duration(Duration::from_secs(90061)), "1d 1h 1m 1s");
    }

    #[test]
    fn format_millis_only() {
        assert_eq!(format_duration(Duration::from_millis(42)), "42ms");
    }

    #[test]
    fn format_seconds_and_millis() {
        assert_eq!(format_duration(Duration::from_millis(1500)), "1s 500ms");
    }

    #[test]
    fn format_large_duration() {
        let d = Duration::from_secs(10 * 86400 + 3 * 3600 + 45 * 60 + 12);
        assert_eq!(format_duration(d), "10d 3h 45m 12s");
    }

    // ---- format_duration_verbose ----

    #[test]
    fn verbose_zero() {
        assert_eq!(format_duration_verbose(Duration::ZERO), "0 seconds");
    }

    #[test]
    fn verbose_singular() {
        assert_eq!(
            format_duration_verbose(Duration::from_secs(86400 + 3600 + 60 + 1)),
            "1 day, 1 hour, 1 minute, 1 second"
        );
    }

    #[test]
    fn verbose_plural() {
        assert_eq!(
            format_duration_verbose(Duration::from_secs(9015)),
            "2 hours, 30 minutes, 15 seconds"
        );
    }

    #[test]
    fn verbose_millis() {
        assert_eq!(
            format_duration_verbose(Duration::from_millis(5)),
            "5 milliseconds"
        );
    }

    #[test]
    fn verbose_one_milli() {
        assert_eq!(
            format_duration_verbose(Duration::from_millis(1)),
            "1 millisecond"
        );
    }

    // ---- format_duration_precise ----

    #[test]
    fn precise_top_two() {
        let d = Duration::from_secs(90061);
        assert_eq!(format_duration_precise(d, 2), "1d 1h");
    }

    #[test]
    fn precise_top_one() {
        let d = Duration::from_secs(90061);
        assert_eq!(format_duration_precise(d, 1), "1d");
    }

    #[test]
    fn precise_all() {
        let d = Duration::from_secs(90061);
        assert_eq!(format_duration_precise(d, 10), "1d 1h 1m 1s");
    }

    #[test]
    fn precise_zero() {
        assert_eq!(format_duration_precise(Duration::ZERO, 2), "0s");
    }

    // ---- parse_duration ----

    #[test]
    fn parse_compact_no_spaces() {
        assert_eq!(
            parse_duration("2h30m").unwrap(),
            Duration::from_secs(9000)
        );
    }

    #[test]
    fn parse_compact_with_spaces() {
        assert_eq!(
            parse_duration("2h 30m 15s").unwrap(),
            Duration::from_secs(9015)
        );
    }

    #[test]
    fn parse_millis() {
        assert_eq!(
            parse_duration("500ms").unwrap(),
            Duration::from_millis(500)
        );
    }

    #[test]
    fn parse_zero() {
        assert_eq!(parse_duration("0s").unwrap(), Duration::ZERO);
    }

    #[test]
    fn parse_days() {
        assert_eq!(
            parse_duration("1d5h").unwrap(),
            Duration::from_secs(86400 + 5 * 3600)
        );
    }

    #[test]
    fn parse_sec_suffix() {
        assert_eq!(
            parse_duration("30sec").unwrap(),
            Duration::from_secs(30)
        );
    }

    #[test]
    fn parse_min_suffix() {
        assert_eq!(
            parse_duration("5min").unwrap(),
            Duration::from_secs(300)
        );
    }

    #[test]
    fn parse_empty() {
        assert_eq!(parse_duration(""), Err(ParseError::EmptyInput));
    }

    #[test]
    fn parse_whitespace_only() {
        assert_eq!(parse_duration("   "), Err(ParseError::EmptyInput));
    }

    #[test]
    fn parse_no_unit() {
        assert!(matches!(
            parse_duration("123"),
            Err(ParseError::InvalidFormat(_))
        ));
    }

    #[test]
    fn parse_unknown_unit() {
        assert!(matches!(
            parse_duration("5w"),
            Err(ParseError::InvalidFormat(_))
        ));
    }

    // ---- parse_duration_verbose ----

    #[test]
    fn parse_verbose_basic() {
        assert_eq!(
            parse_duration_verbose("2 hours, 30 minutes").unwrap(),
            Duration::from_secs(9000)
        );
    }

    #[test]
    fn parse_verbose_no_commas() {
        assert_eq!(
            parse_duration_verbose("1 day 5 hours").unwrap(),
            Duration::from_secs(86400 + 5 * 3600)
        );
    }

    #[test]
    fn parse_verbose_singular() {
        assert_eq!(
            parse_duration_verbose("1 hour, 1 minute, 1 second").unwrap(),
            Duration::from_secs(3661)
        );
    }

    #[test]
    fn parse_verbose_millis() {
        assert_eq!(
            parse_duration_verbose("500 milliseconds").unwrap(),
            Duration::from_millis(500)
        );
    }

    #[test]
    fn parse_verbose_empty() {
        assert_eq!(
            parse_duration_verbose(""),
            Err(ParseError::EmptyInput)
        );
    }

    // ---- round-trip ----

    #[test]
    fn roundtrip_compact() {
        let original = Duration::from_secs(9015);
        let formatted = format_duration(original);
        let parsed = parse_duration(&formatted).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn roundtrip_with_millis() {
        let original = Duration::from_millis(90015_042);
        let formatted = format_duration(original);
        let parsed = parse_duration(&formatted).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn roundtrip_zero() {
        let formatted = format_duration(Duration::ZERO);
        let parsed = parse_duration(&formatted).unwrap();
        assert_eq!(parsed, Duration::ZERO);
    }

    #[test]
    fn roundtrip_verbose() {
        let original = Duration::from_secs(90061);
        let formatted = format_duration_verbose(original);
        let parsed = parse_duration_verbose(&formatted).unwrap();
        assert_eq!(parsed, original);
    }

    // ---- edge cases ----

    #[test]
    fn very_large_duration() {
        let d = Duration::from_secs(365 * 86400);
        let formatted = format_duration(d);
        assert_eq!(formatted, "365d");
        let parsed = parse_duration(&formatted).unwrap();
        assert_eq!(parsed, d);
    }

    #[test]
    fn parse_with_extra_whitespace() {
        assert_eq!(
            parse_duration("  2h   30m  ").unwrap(),
            Duration::from_secs(9000)
        );
    }

    // ---- format_duration_iso8601 ----

    #[test]
    fn iso8601_zero() {
        assert_eq!(format_duration_iso8601(Duration::ZERO), "PT0S");
    }

    #[test]
    fn iso8601_hours_minutes_seconds() {
        assert_eq!(
            format_duration_iso8601(Duration::from_secs(9015)),
            "PT2H30M15S"
        );
    }

    #[test]
    fn iso8601_hours_only() {
        assert_eq!(
            format_duration_iso8601(Duration::from_secs(7200)),
            "PT2H"
        );
    }

    #[test]
    fn iso8601_minutes_only() {
        assert_eq!(
            format_duration_iso8601(Duration::from_secs(300)),
            "PT5M"
        );
    }

    #[test]
    fn iso8601_seconds_only() {
        assert_eq!(
            format_duration_iso8601(Duration::from_secs(45)),
            "PT45S"
        );
    }

    // ---- parse_iso8601_duration ----

    #[test]
    fn parse_iso8601_full() {
        assert_eq!(
            parse_iso8601_duration("PT2H30M15S").unwrap(),
            Duration::from_secs(9015)
        );
    }

    #[test]
    fn parse_iso8601_minutes_only() {
        assert_eq!(
            parse_iso8601_duration("PT5M").unwrap(),
            Duration::from_secs(300)
        );
    }

    #[test]
    fn parse_iso8601_zero() {
        assert_eq!(
            parse_iso8601_duration("PT0S").unwrap(),
            Duration::ZERO
        );
    }

    #[test]
    fn parse_iso8601_missing_prefix() {
        assert!(matches!(
            parse_iso8601_duration("2H30M"),
            Err(ParseError::InvalidFormat(_))
        ));
    }

    #[test]
    fn parse_iso8601_empty() {
        assert_eq!(parse_iso8601_duration(""), Err(ParseError::EmptyInput));
    }

    #[test]
    fn roundtrip_iso8601() {
        let original = Duration::from_secs(9015);
        let formatted = format_duration_iso8601(original);
        let parsed = parse_iso8601_duration(&formatted).unwrap();
        assert_eq!(parsed, original);
    }

    // ---- format_duration_short ----

    #[test]
    fn short_zero() {
        assert_eq!(format_duration_short(Duration::ZERO), "0s");
    }

    #[test]
    fn short_hours_minutes_seconds() {
        assert_eq!(
            format_duration_short(Duration::from_secs(9015)),
            "2h30m15s"
        );
    }

    #[test]
    fn short_seconds_only() {
        assert_eq!(format_duration_short(Duration::from_secs(45)), "45s");
    }

    #[test]
    fn short_with_days() {
        assert_eq!(
            format_duration_short(Duration::from_secs(90061)),
            "1d1h1m1s"
        );
    }

    #[test]
    fn short_with_millis() {
        assert_eq!(
            format_duration_short(Duration::from_millis(1500)),
            "1s500ms"
        );
    }
}
