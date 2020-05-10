#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use unicode_segmentation::UnicodeSegmentation;

pub fn limit(s: &str, limit: usize) -> String {
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
    fn limit() {
        let s = "Even Olsson Rogstadkjærnet";
        assert_eq!(crate::limit(s, 26), "Even Olsson Rogstadkjærnet");
        assert_eq!(crate::limit(s, 25), "Even Olsson R");
        assert_eq!(crate::limit(s, 12), "Even O R");
        assert_eq!(crate::limit(s, 7), "E O R");
        assert_eq!(crate::limit(s, 4), "EOR");
        assert_eq!(crate::limit(s, 2), "EO");
        assert_eq!(crate::limit(s, 1), "E");
        assert_eq!(crate::limit(s, 0), "");
    }
}
