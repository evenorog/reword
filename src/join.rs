use alloc::string::String;

/// The conjunction and serial-comma convention for [`join_with`].
///
/// The default is `"and"` without a serial comma. Items are otherwise preserved
/// as provided, including empty strings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JoinOptions<'a> {
    /// Word inserted before the last item, with one space on each side.
    ///
    /// Use an empty string for a plain comma-separated list.
    pub conjunction: &'a str,
    /// Insert a comma before the conjunction for lists of at least three items.
    ///
    /// Has no effect when the conjunction is empty.
    pub serial_comma: bool,
}

impl Default for JoinOptions<'_> {
    fn default() -> Self {
        Self {
            conjunction: "and",
            serial_comma: false,
        }
    }
}

/// Join the list with an 'or' before the last element of the list.
///
/// # Examples
/// ```
/// assert_eq!(reword::or_join(&["a", "b"]), "a or b");
/// assert_eq!(reword::or_join(&["a", "b", "c"]), "a, b or c");
/// ```
pub fn or_join<I>(iter: I) -> String
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    join_with(
        iter,
        JoinOptions {
            conjunction: "or",
            ..JoinOptions::default()
        },
    )
}

/// Join the list with an 'and' before the last element of the list.
///
/// # Examples
/// ```
/// assert_eq!(reword::and_join(&["a", "b"]), "a and b");
/// assert_eq!(reword::and_join(&["a", "b", "c"]), "a, b and c");
/// ```
pub fn and_join<I>(iter: I) -> String
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    join_with(iter, JoinOptions::default())
}

/// Joins items with commas and a configurable conjunction and serial comma.
///
/// An empty input returns an empty string; a single item is returned as provided.
/// A serial comma is only added for three or more items. The conjunction is
/// inserted verbatim with one surrounding space on each side; an empty
/// conjunction produces a plain comma-separated list.
///
/// Accepts one-shot iterators without collecting the input.
///
/// # Examples
/// ```
/// use reword::{JoinOptions, join_with};
///
/// let names = ["Alice", "Bob", "Charlie"];
/// assert_eq!(join_with(names, JoinOptions::default()), "Alice, Bob and Charlie");
/// assert_eq!(
///     join_with(names, JoinOptions { serial_comma: true, ..JoinOptions::default() }),
///     "Alice, Bob, and Charlie"
/// );
/// assert_eq!(
///     join_with(names, JoinOptions { conjunction: "og", serial_comma: false }),
///     "Alice, Bob og Charlie"
/// );
/// ```
pub fn join_with<I>(iter: I, options: JoinOptions<'_>) -> String
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut iter = iter.into_iter();
    let Some(first) = iter.next() else {
        return String::new();
    };

    let mut string = String::new();
    string.push_str(first.as_ref());
    let Some(mut next) = iter.next() else {
        return string;
    };

    let mut has_middle = false;
    for peek in iter {
        string.push_str(", ");
        string.push_str(next.as_ref());
        next = peek;
        has_middle = true;
    }

    if options.conjunction.is_empty() {
        string.push_str(", ");
    } else {
        if has_middle && options.serial_comma {
            string.push(',');
        }
        string.push(' ');
        string.push_str(options.conjunction);
        string.push(' ');
    }
    string.push_str(next.as_ref());
    string
}
