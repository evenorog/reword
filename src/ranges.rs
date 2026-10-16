use alloc::string::String;
use core::fmt::{Display, Write};
use num_traits::{CheckedAdd, PrimInt};

/// Formats consecutive ascending integer values as comma-separated ranges.
///
/// Runs of two or more values use an en dash (`–`). Adjacent duplicates are
/// ignored. Input order is preserved: a decreasing value or a gap starts a new
/// range. Sorted input produces the most compact representation; this function
/// does not sort or collect the input.
///
/// Accepts one-shot iterators of any primitive signed or unsigned integer type.
/// For borrowed slices or iterators, use `.iter().copied()` to supply values.
/// Empty input returns an empty string. Integer boundaries are checked without
/// overflow, including the full `i128` and `u128` ranges.
///
/// # Examples
/// ```
/// assert_eq!(reword::format_ranges([1, 2, 3, 7, 9, 10]), "1–3, 7, 9–10");
/// assert_eq!(reword::format_ranges([1, 1, 2, 4]), "1–2, 4");
/// assert_eq!(reword::format_ranges([-3, -2, -1, 1, 2]), "-3–-1, 1–2");
/// assert_eq!(reword::format_ranges([3, 1, 2]), "3, 1–2");
///
/// let values = [1_u64, 2, 3];
/// assert_eq!(reword::format_ranges(values.iter().copied()), "1–3");
/// ```
pub fn format_ranges<I>(values: I) -> String
where
    I: IntoIterator,
    I::Item: PrimInt + Display,
{
    let mut values = values.into_iter();
    let Some(mut start) = values.next() else {
        return String::new();
    };
    let mut end = start;
    let one = <I::Item as num_traits::One>::one();
    let mut output = String::new();

    for next in values {
        if next == end {
            continue;
        }
        if end.checked_add(&one) == Some(next) {
            end = next;
        } else {
            append_range(&mut output, start, end);
            start = next;
            end = next;
        }
    }
    append_range(&mut output, start, end);
    output
}

fn append_range<T: Copy + Eq + Display>(output: &mut String, start: T, end: T) {
    if !output.is_empty() {
        output.push_str(", ");
    }
    write!(output, "{start}").expect("writing to a String cannot fail");
    if start != end {
        write!(output, "–{end}").expect("writing to a String cannot fail");
    }
}
