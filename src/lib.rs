//! Provides some utility functions for human-readable formatting of words.
//!
//! Case conversion uses [`heck`]'s word-boundary and Unicode casing rules.
//! Use [`abbreviate_name`] to shorten a name to a maximum number of grapheme
//! clusters. Abbreviation can be composed with case conversion or [`username`];
//! those subsequent operations may change the output length.
//! Use [`truncate`] for an allocation-free prefix limited by grapheme count.
//! Format byte counts with [`bytes`], numbers with [`number`], percentages with
//! [`percent`], rates with [`bytes_per_second`], and elapsed time with
//! [`duration_short`].

#![no_std]

extern crate alloc;

mod case;
mod join;
mod name;
mod text;
mod units;

pub use case::*;
pub use join::*;
pub use name::*;
pub use text::*;
pub use units::*;

use alloc::string::String;

fn fold(mut acc: String, w: &str, ch: char) -> String {
    if !acc.is_empty() {
        acc.push(ch);
    }
    acc.push_str(w);
    acc
}
