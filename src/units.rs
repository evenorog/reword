use alloc::format;
use alloc::string::String;
use core::fmt::Write;

/// Formats byte counts using 1024-based units (B, KB, MB, etc.).
///
/// # Examples
///
/// ```
/// assert_eq!(reword::bytes(0), "0 B");
/// assert_eq!(reword::bytes(1024), "1.0 KB");
/// assert_eq!(reword::bytes(1048576), "1.0 MB");
/// ```
pub fn bytes(n: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    if n == 0 {
        return "0 B".into();
    }
    let mut n = n as f64;
    let mut unit_idx = 0;
    while n >= 1024.0 && unit_idx < units.len() - 1 {
        n /= 1024.0;
        unit_idx += 1;
    }
    let mut res = String::with_capacity(16);
    if unit_idx == 0 {
        let _ = write!(res, "{n} {}", units[unit_idx]);
    } else {
        let _ = write!(res, "{n:.1} {}", units[unit_idx]);
    }
    res
}

/// Formats numbers with thousands separators (commas).
///
/// # Examples
///
/// ```
/// assert_eq!(reword::number(1234.56), "1,234.56");
/// assert_eq!(reword::number(-10000.0), "-10,000");
/// ```
pub fn number(n: f64) -> String {
    let s = format!("{n}");
    let (int_part, frac_part) = if let Some(pos) = s.find('.') {
        (&s[..pos], Some(&s[pos + 1..]))
    } else {
        (s.as_str(), None)
    };

    let is_negative = int_part.starts_with('-');
    let abs_int = if is_negative { &int_part[1..] } else { int_part };

    let mut final_s = String::with_capacity(s.len() + s.len() / 3 + 1);
    if is_negative {
        final_s.push('-');
    }

    let bytes = abs_int.as_bytes();
    let len = bytes.len();
    for (i, &byte) in bytes.iter().enumerate() {
        let remaining = len - i;
        if i > 0 && remaining % 3 == 0 {
            final_s.push(',');
        }
        final_s.push(byte as char);
    }

    if let Some(frac) = frac_part {
        final_s.push('.');
        final_s.push_str(frac);
    }
    final_s
}

/// Formats numbers using short suffixes (K, M, B, T).
///
/// # Examples
///
/// ```
/// assert_eq!(reword::number_short(1500.0), "1.5K");
/// assert_eq!(reword::number_short(1000000.0), "1M");
/// ```
pub fn number_short(n: f64) -> String {
    let units = ["", "K", "M", "B", "T"];
    let mut n = n;
    let mut unit_idx = 0;
    while (if n < 0.0 { -n } else { n }) >= 1000.0 && unit_idx < units.len() - 1 {
        n /= 1000.0;
        unit_idx += 1;
    }
    let mut res = String::with_capacity(16);
    if unit_idx == 0 {
        let _ = write!(res, "{n}");
    } else if n == (n as i64) as f64 {
        let _ = write!(res, "{n}{}", units[unit_idx]);
    } else {
        let _ = write!(res, "{n:.1}{}", units[unit_idx]);
    }
    res
}

/// Formats durations in seconds into days, hours, minutes, and seconds.
///
/// # Examples
///
/// ```
/// assert_eq!(reword::duration(3661), "1h 1m 1s");
/// assert_eq!(reword::duration(86401), "1d 0h 0m 1s");
/// ```
pub fn duration(seconds: u64) -> String {
    if seconds == 0 {
        return "0s".into();
    }
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    let mut res = String::with_capacity(32);
    if days > 0 {
        let _ = write!(res, "{}d", days);
    }
    if days > 0 || hours > 0 {
        if !res.is_empty() {
            res.push(' ');
        }
        let _ = write!(res, "{}h", hours);
    }
    if days > 0 || hours > 0 || minutes > 0 {
        if !res.is_empty() {
            res.push(' ');
        }
        let _ = write!(res, "{}m", minutes);
    }
    if !res.is_empty() {
        res.push(' ');
    }
    let _ = write!(res, "{}s", seconds);

    res
}
