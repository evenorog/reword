//! Provides functions for human readable formatting of words and sentences.
//!
//! ```
//! let s = "(Even),Olsson&Rogstadkjærnet?";
//! assert_eq!(reword::name(s), "Even Olsson Rogstadkjærnet");
//! assert_eq!(reword::name_with_limit(s, 4), "EOR");
//! ```

#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use unicode_segmentation::UnicodeSegmentation;

/// Formats the input string as a name.
///
/// # Examples
/// ```
/// assert_eq!(reword::name("(Even),Olsson&Rogstadkjærnet?"), "Even Olsson Rogstadkjærnet");
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
/// assert_eq!(reword::name_with_limit("(Even),Olsson&Rogstadkjærnet?", 4), "EOR");
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
    for (w, c) in name.iter_mut().zip(n).rev() {
        if count <= limit {
            break;
        }
        count -= c - 1;
        *w = w.graphemes(true).next().unwrap();
    }

    if count <= limit {
        name.join(" ")
    } else if (count - spaces) <= limit {
        name.concat()
    } else {
        name[..limit].concat()
    }
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
        assert_eq!(crate::name_with_limit(s, 25), "Even Olsson R");
        assert_eq!(crate::name_with_limit(s, 12), "Even O R");
        assert_eq!(crate::name_with_limit(s, 7), "E O R");
        assert_eq!(crate::name_with_limit(s, 4), "EOR");
        assert_eq!(crate::name_with_limit(s, 2), "EO");
        assert_eq!(crate::name_with_limit(s, 1), "E");
        assert_eq!(crate::name_with_limit(s, 0), "");
    }

    #[test]
    fn join() {
        assert_eq!(crate::or_join::<&str>(&[]), "");
        assert_eq!(crate::or_join::<&str>(&["a"]), "a");
        assert_eq!(crate::or_join::<&str>(&["a", "b"]), "a or b");
        assert_eq!(crate::or_join::<&str>(&["a", "b", "c"]), "a, b, or c");
        assert_eq!(crate::or_join::<&str>(&["a", "b", "c", "d", "e"]), "a, b, c, d, or e");
        assert_eq!(crate::and_join::<&str>(&[]), "");
        assert_eq!(crate::and_join::<&str>(&["a"]), "a");
        assert_eq!(crate::and_join::<&str>(&["a", "b"]), "a and b");
        assert_eq!(crate::and_join::<&str>(&["a", "b", "c"]), "a, b, and c");
        assert_eq!(crate::and_join::<&str>(&["a", "b", "c", "d", "e"]), "a, b, c, d, and e");
    }
}
