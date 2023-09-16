//! Provides some utility functions for human-readable formatting of words.

#![no_std]

extern crate alloc;

mod cases;

pub use cases::*;

use alloc::string::String;
use alloc::vec::Vec;
use unicode_segmentation::UnicodeSegmentation;

/// Formats the input string as a name.
///
/// # Examples
/// ```
/// let t = "(Even),Olsson&Rogstadkjærnet?";
/// assert_eq!(reword::name(t), "Even Olsson Rogstadkjærnet");
/// ```
pub fn name<T: AsRef<str>>(t: T) -> String {
    t.as_ref()
        .unicode_words()
        .fold(String::new(), |acc, w| fold(acc, w, ' '))
}

/// Formats the input string as a name and limits the length of the name.
///
/// # Examples
/// ```
/// let t = "(Even),Olsson&Rogstadkjærnet?";
/// assert_eq!(reword::name_with_limit(t, 4), "EOR");
/// assert_eq!(reword::name_with_limit(t, 25), "Even O Rogstadkjærnet");
/// ```
pub fn name_with_limit<T: AsRef<str>>(t: T, limit: usize) -> String {
    let mut name: Vec<&str> = t.as_ref().unicode_words().collect();
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
/// assert_eq!(reword::username("Even Olsson Rogstadkjærnet"), "evenolssonrogstadkjærnet");
/// ```
pub fn username<T: AsRef<str>>(t: T) -> String {
    name(t)
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

/// Creates a username from the provided string and limit.
///
/// A username can only consist of alphanumeric characters.
///
/// # Examples
/// ```
/// assert_eq!(reword::username_with_limit("Even Olsson Rogstadkjærnet", 25), "evenorogstadkjærnet");
/// ```
pub fn username_with_limit<T: AsRef<str>>(t: T, limit: usize) -> String {
    name_with_limit(t, limit)
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
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

fn fold(mut acc: String, w: &str, ch: char) -> String {
    if !acc.is_empty() {
        acc.push(ch);
    }
    acc.push_str(w);
    acc
}
