//! Provides nice formatting of names.
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
pub fn name(s: &str) -> String {
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
pub fn name_with_limit(s: &str, limit: usize) -> String {
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
pub fn or_join(v: &[impl AsRef<str>]) -> String {
    join(v, ", or ")
}

/// Join the list with an 'and' before the last element of the list.
pub fn and_join(v: &[impl AsRef<str>]) -> String {
    join(v, ", and ")
}

fn join(v: &[impl AsRef<str>], sep: &str) -> String {
    let mut s = String::new();
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
}
