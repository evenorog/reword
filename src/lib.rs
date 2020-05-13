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
    let s: Vec<&str> = s.unicode_words().collect();
    s.join(" ")
}

/// Formats the input string as a name and limits the length of the name.
pub fn name_with_limit(s: &str, limit: usize) -> String {
    let mut s: Vec<&str> = s.unicode_words().collect();
    if s.is_empty() {
        return String::new();
    }

    let n: Vec<usize> = s.iter().map(|w| w.graphemes(true).count()).collect();
    let spaces = s.len() - 1;
    let mut count = n.iter().sum::<usize>() + spaces;
    for (w, c) in s.iter_mut().zip(n).rev() {
        if count <= limit {
            break;
        }
        count -= c - 1;
        *w = w.graphemes(true).next().unwrap();
    }

    if count <= limit {
        s.join(" ")
    } else if (count - spaces) <= limit {
        s.concat()
    } else {
        s[..limit].concat()
    }
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
