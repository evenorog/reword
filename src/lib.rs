//! Provides utility functions for human readable formatting of words and sentences.
//!
//! ```
//! let s = "(Even),Olsson&Rogstadkjærnet?";
//! assert_eq!(reword::name(s), "Even Olsson Rogstadkjærnet");
//! assert_eq!(reword::name_with_limit(s, 4), "EOR");
//! assert_eq!(reword::username_with_limit(s, 12), "evenor");
//! ```

#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use unicode_segmentation::UnicodeSegmentation;

/// Formats the input string as a name.
///
/// # Examples
/// ```
/// let s = "(Even),Olsson&Rogstadkjærnet?";
/// assert_eq!(reword::name(s), "Even Olsson Rogstadkjærnet");
/// ```
pub fn name<T: AsRef<str>>(s: T) -> String {
    let s = s.as_ref();
    let mut name = String::with_capacity(s.len());
    let mut it = s.unicode_words();
    if let Some(w) = it.next() {
        name.push_str(w);
        for w in it {
            name.push(' ');
            name.push_str(w)
        }
    }
    name
}

/// Formats the input string as a name and limits the length of the name.
///
/// # Examples
/// ```
/// let s = "(Even),Olsson&Rogstadkjærnet?";
/// assert_eq!(reword::name_with_limit(s, 4), "EOR");
/// assert_eq!(reword::name_with_limit(s, 25), "Even O Rogstadkjærnet");
/// ```
pub fn name_with_limit<T: AsRef<str>>(s: T, limit: usize) -> String {
    let s = s.as_ref();
    let mut name: Vec<&str> = s.unicode_words().collect();
    if name.is_empty() {
        return String::new();
    }

    let len = name.len();
    let mut n = Vec::with_capacity(len);
    let mut sum = 0;
    for &w in &name {
        let c = w.graphemes(true).count();
        sum += c;
        n.push(c);
    }

    let spaces = len - 1;
    let mut count = sum + spaces;
    let mut zip = name.iter_mut().zip(n);

    // Pop off the first name, so we can check that last.
    let head = zip.next();

    // Checks if the words needs to be shortened, starting with the first middle name.
    for (w, c) in zip {
        if count <= limit {
            break;
        }
        count -= c - 1;
        *w = w.graphemes(true).next().unwrap();
    }

    // Checks if the first name also needs to be shortened.
    if let Some((w, c)) = head {
        if count > limit {
            count -= c - 1;
            *w = w.graphemes(true).next().unwrap();
        }
    }

    if count <= limit {
        name.join(" ")
    } else if (count - spaces) <= limit {
        name.concat()
    } else {
        name[..limit].concat()
    }
}

/// Creates a username from the provided string.
///
/// A username can only consist of alphanumeric characters.
///
/// # Examples
/// ```
/// assert_eq!(reword::username("Even O. R."), "evenor");
/// ```
pub fn username<T: AsRef<str>>(s: T) -> String {
    let s = s.as_ref();
    let mut username = String::with_capacity(s.len());
    for c in s.chars().filter(char::is_ascii_alphanumeric) {
        let c = c.to_ascii_lowercase();
        username.push(c);
    }
    username
}

/// Creates a username from the provided string and limit.
///
/// A username can only consist of alphanumeric characters.
///
/// # Examples
/// ```
/// assert_eq!(reword::username_with_limit("Even Olsson Rogstadkjærnet", 12), "evenor");
/// ```
pub fn username_with_limit<T: AsRef<str>>(s: T, limit: usize) -> String {
    let name = name_with_limit(s, limit);
    let mut username = String::with_capacity(name.len());
    for c in name.chars().filter(char::is_ascii_alphanumeric) {
        let c = c.to_ascii_lowercase();
        username.push(c)
    }
    username
}

/// Join the list with an 'or' before the last element of the list.
///
/// # Examples
/// ```
/// assert_eq!(reword::or_join(&["a", "b"]), "a or b");
/// assert_eq!(reword::or_join(&["a", "b", "c"]), "a, b, or c");
/// ```
pub fn or_join<T: AsRef<str>>(v: &[T]) -> String {
    if v.len() < 3 {
        join(v, " or ")
    } else {
        join(v, ", or ")
    }
}

/// Join the list with an 'and' before the last element of the list.
///
/// # Examples
/// ```
/// assert_eq!(reword::and_join(&["a", "b"]), "a and b");
/// assert_eq!(reword::and_join(&["a", "b", "c"]), "a, b, and c");
/// ```
pub fn and_join<T: AsRef<str>>(v: &[T]) -> String {
    if v.len() < 3 {
        join(v, " and ")
    } else {
        join(v, ", and ")
    }
}

fn join<T: AsRef<str>>(v: &[T], sep: &str) -> String {
    let mut s = String::with_capacity(v.len() * 16);
    if let Some((first, tail)) = v.split_first() {
        s.push_str(first.as_ref());
        if let Some((last, tail)) = tail.split_last() {
            for field in tail {
                s.push_str(", ");
                s.push_str(field.as_ref());
            }
            s.push_str(sep);
            s.push_str(last.as_ref());
        }
    }
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn name() {
        let s = "(Even), Olsson&Rogstadkjærnet?";
        assert_eq!(crate::name(s), "Even Olsson Rogstadkjærnet");
        assert_eq!(crate::name_with_limit(s, 25), "Even O Rogstadkjærnet");
        assert_eq!(crate::name_with_limit(s, 12), "Even O R");
        assert_eq!(crate::name_with_limit(s, 7), "E O R");
        assert_eq!(crate::name_with_limit(s, 4), "EOR");
        assert_eq!(crate::name_with_limit(s, 2), "EO");
        assert_eq!(crate::name_with_limit(s, 1), "E");
        assert_eq!(crate::name_with_limit(s, 0), "");
    }

    #[test]
    fn username() {
        let s = "(Even), Olsson&Rogstadkjærnet?";
        assert_eq!(crate::username(s), "evenolssonrogstadkjrnet");
        assert_eq!(crate::username_with_limit(s, 25), "evenorogstadkjrnet");
        assert_eq!(crate::username_with_limit(s, 12), "evenor");
        assert_eq!(crate::username_with_limit(s, 7), "eor");
        assert_eq!(crate::username_with_limit(s, 4), "eor");
        assert_eq!(crate::username_with_limit(s, 2), "eo");
        assert_eq!(crate::username_with_limit(s, 1), "e");
        assert_eq!(crate::username_with_limit(s, 0), "");
    }

    #[test]
    fn join() {
        assert_eq!(crate::or_join::<&str>(&[]), "");
        assert_eq!(crate::or_join::<&str>(&["a"]), "a");
        assert_eq!(crate::or_join::<&str>(&["a", "b"]), "a or b");
        assert_eq!(crate::or_join::<&str>(&["a", "b", "c"]), "a, b, or c");
        assert_eq!(
            crate::or_join::<&str>(&["a", "b", "c", "d", "e"]),
            "a, b, c, d, or e"
        );
        assert_eq!(crate::and_join::<&str>(&[]), "");
        assert_eq!(crate::and_join::<&str>(&["a"]), "a");
        assert_eq!(crate::and_join::<&str>(&["a", "b"]), "a and b");
        assert_eq!(crate::and_join::<&str>(&["a", "b", "c"]), "a, b, and c");
        assert_eq!(
            crate::and_join::<&str>(&["a", "b", "c", "d", "e"]),
            "a, b, c, d, and e"
        );
    }
}
