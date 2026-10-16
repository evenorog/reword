//! Human-readable byte counts, numbers, and elapsed seconds.
//!
//! All helpers return [`String`] and use fixed, locale-independent conventions.
//! [`bytes`] uses binary IEC prefixes; [`bytes_decimal`] uses decimal SI prefixes.
//! [`number`] groups decimal digits with commas, while [`number_short`] uses
//! English suffixes for thousands, millions, billions, and trillions.

use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::{Display, Write};

/// Formats a byte count using powers of 1024: B, KiB, MiB, GiB, TiB, PiB, EiB.
///
/// Byte counts below 1024 are exact integers. Larger values are rounded to the
/// nearest tenth, with halves rounded up. Rounding to 1024 promotes the result
/// to the next unit. All `u64` values are supported using integer arithmetic.
///
/// # Examples
/// ```
/// assert_eq!(reword::units::bytes(0), "0 B");
/// assert_eq!(reword::units::bytes(1023), "1023 B");
/// assert_eq!(reword::units::bytes(1024), "1.0 KiB");
/// assert_eq!(reword::units::bytes(1536), "1.5 KiB");
/// ```
pub fn bytes(n: u64) -> String {
    format_bytes(n, 1024, &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"])
}

/// Formats a byte count using powers of 1000: B, kB, MB, GB, TB, PB, EB.
///
/// Uses the same rounding rules as [`bytes`], with decimal rather than binary
/// units. Counts below 1000 are exact integers.
///
/// # Examples
/// ```
/// assert_eq!(reword::units::bytes_decimal(1000), "1.0 kB");
/// assert_eq!(reword::units::bytes_decimal(1500), "1.5 kB");
/// assert_eq!(reword::units::bytes_decimal(1_000_000), "1.0 MB");
/// ```
pub fn bytes_decimal(n: u64) -> String {
    format_bytes(n, 1000, &["B", "kB", "MB", "GB", "TB", "PB", "EB"])
}

fn format_bytes(n: u64, base: u64, units: &[&str; 7]) -> String {
    if n < base {
        return format!("{n} B");
    }

    let mut unit = 1;
    let mut divisor = base;
    while unit < units.len() - 1 && n / divisor >= base {
        divisor *= base;
        unit += 1;
    }

    // Widen before multiplying so even u64::MAX can be rounded exactly.
    let round_tenths =
        |divisor: u64| (u128::from(n) * 10 + u128::from(divisor) / 2) / u128::from(divisor);
    let mut tenths = round_tenths(divisor);
    if unit < units.len() - 1 && tenths >= u128::from(base) * 10 {
        divisor *= base;
        unit += 1;
        tenths = round_tenths(divisor);
    }

    let mut output = String::with_capacity(16);
    write!(output, "{}.{} {}", tenths / 10, tenths % 10, units[unit])
        .expect("writing to a String cannot fail");
    output
}

/// Formats a displayed decimal number with comma-separated groups of three.
///
/// Accepts integers, floating-point numbers, and other [`Display`] values such
/// as decimal types or numeric strings. The original decimal representation is
/// used without converting integers to floating point or rounding their digits.
/// A sign, fractional part, and scientific exponent are preserved unchanged.
///
/// The conventions are fixed: commas separate thousands and a dot separates
/// fractional digits. Text that is not a decimal number, including `NaN` and
/// infinities, is returned unchanged. Floating-point inputs retain the precision
/// of their own [`Display`] implementation, including negative zero.
///
/// # Examples
/// ```
/// assert_eq!(reword::units::number(1234.56), "1,234.56");
/// assert_eq!(reword::units::number(-10000), "-10,000");
/// assert_eq!(reword::units::number(9_007_199_254_740_993_u64), "9,007,199,254,740,993");
/// assert_eq!(reword::units::number("12345.670e+8"), "12,345.670e+8");
/// ```
pub fn number<T: Display>(n: T) -> String {
    let text = n.to_string();
    let Some((start, end)) = integer_digits(&text) else {
        return text;
    };
    let digits = end - start;
    if digits <= 3 {
        return text;
    }

    let mut output = String::with_capacity(text.len() + (digits - 1) / 3);
    output.push_str(&text[..start]);
    for (i, digit) in text[start..end].bytes().enumerate() {
        if i > 0 && (digits - i).is_multiple_of(3) {
            output.push(',');
        }
        output.push(char::from(digit));
    }
    output.push_str(&text[end..]);
    output
}

// Validate the whole decimal representation before inserting separators.
fn integer_digits(text: &str) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    let start = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    let mut pos = start;
    while bytes.get(pos).is_some_and(u8::is_ascii_digit) {
        pos += 1;
    }
    let end = pos;
    if end == start {
        return None;
    }

    if bytes.get(pos) == Some(&b'.') {
        pos += 1;
        while bytes.get(pos).is_some_and(u8::is_ascii_digit) {
            pos += 1;
        }
    }
    if matches!(bytes.get(pos), Some(b'e' | b'E')) {
        pos += 1;
        if matches!(bytes.get(pos), Some(b'+' | b'-')) {
            pos += 1;
        }
        let exponent_start = pos;
        while bytes.get(pos).is_some_and(u8::is_ascii_digit) {
            pos += 1;
        }
        if pos == exponent_start {
            return None;
        }
    }
    (pos == bytes.len()).then_some((start, end))
}

/// Formats a number using English short suffixes: K, M, B, T.
///
/// Finite magnitudes below 1000 use the normal floating-point representation.
/// Larger magnitudes are scaled by powers of 1000 and rounded to one decimal
/// place using Rust's floating-point formatting. A trailing `.0` is omitted;
/// rounding to 1000 promotes the suffix when another suffix is available.
/// Trillions are the largest unit, so still larger values keep the T suffix.
///
/// Negative numbers use the same magnitude-based rules. `NaN`, infinities, and
/// negative zero use their normal floating-point representations. This is an
/// approximate display formatter; use [`number`] to preserve integer digits.
///
/// # Examples
/// ```
/// assert_eq!(reword::units::number_short(1500.0), "1.5K");
/// assert_eq!(reword::units::number_short(1_000_000.0), "1M");
/// assert_eq!(reword::units::number_short(999_999.0), "1M");
/// assert_eq!(reword::units::number_short(-2500.0), "-2.5K");
/// ```
pub fn number_short(n: f64) -> String {
    if !n.is_finite() || n.abs() < 1000.0 {
        return n.to_string();
    }

    const SUFFIXES: [&str; 5] = ["", "K", "M", "B", "T"];
    let mut scaled = n;
    let mut unit = 0;
    while unit < SUFFIXES.len() - 1 && scaled.abs() >= 1000.0 {
        scaled /= 1000.0;
        unit += 1;
    }

    let mut output = String::with_capacity(16);
    write!(output, "{scaled:.1}").expect("writing to a String cannot fail");
    if unit < SUFFIXES.len() - 1 && matches!(output.as_str(), "1000.0" | "-1000.0") {
        scaled /= 1000.0;
        unit += 1;
        output.clear();
        write!(output, "{scaled:.1}").expect("writing to a String cannot fail");
    }
    if output.ends_with(".0") {
        output.truncate(output.len() - 2);
    }
    output.push_str(SUFFIXES[unit]);
    output
}

/// Formats whole elapsed seconds as days, hours, minutes, and seconds.
///
/// A day is exactly 86,400 seconds. All components from the largest nonzero
/// unit down to seconds are included, even if an intermediate component is zero.
/// Zero is `"0s"`. The full range of `u64` seconds is supported.
///
/// For a [`core::time::Duration`], pass its whole seconds using `as_secs()`;
/// fractional seconds are discarded by that conversion.
///
/// # Examples
/// ```
/// assert_eq!(reword::units::duration(3661), "1h 1m 1s");
/// assert_eq!(reword::units::duration(86401), "1d 0h 0m 1s");
/// assert_eq!(reword::units::duration(0), "0s");
/// ```
pub fn duration(seconds: u64) -> String {
    let days = seconds / 86_400;
    let hours = seconds % 86_400 / 3600;
    let minutes = seconds % 3600 / 60;
    let seconds = seconds % 60;

    let mut output = String::with_capacity(32);
    if days > 0 {
        write!(output, "{days}d {hours}h {minutes}m {seconds}s")
    } else if hours > 0 {
        write!(output, "{hours}h {minutes}m {seconds}s")
    } else if minutes > 0 {
        write!(output, "{minutes}m {seconds}s")
    } else {
        write!(output, "{seconds}s")
    }
    .expect("writing to a String cannot fail");
    output
}
