use alloc::string::String;

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
    join(iter, ", ", " or ")
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
    join(iter, ", ", " and ")
}

fn join<I>(iter: I, mid_sep: &str, end_sep: &str) -> String
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut iter = iter.into_iter();
    let Some(first) = iter.next() else {
        return String::new();
    };

    let (lower, upper) = iter.size_hint();
    let mut string = String::with_capacity(upper.unwrap_or(lower) * 8);
    string.push_str(first.as_ref());
    let Some(mut next) = iter.next() else {
        return string;
    };

    for peek in iter {
        string.push_str(mid_sep);
        string.push_str(next.as_ref());
        next = peek;
    }

    string.push_str(end_sep);
    string.push_str(next.as_ref());
    string
}
