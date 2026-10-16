//! Provides some utility functions for human-readable formatting of words.
//!
//! Case conversion uses [`heck`]'s word-boundary and Unicode casing rules.
//! Use [`abbreviate_name`] to shorten a name to a maximum number of grapheme
//! clusters. Abbreviation can be composed with case conversion or [`username`];
//! those subsequent operations may change the output length.
//! Use [`username_ascii`] for username candidates transliterated to ASCII.
//! Use [`ascii`] to transliterate text while preserving case and punctuation.
//! Use [`truncate`] for an allocation-free prefix limited by grapheme count.
//! Format byte counts with [`bytes`], numbers with [`number`], percentages with
//! [`percent`], rates with [`bytes_per_second`], and elapsed time with
//! [`duration_short`].
//! Create ASCII slugs with [`slug_ascii`] and compact integer sequences with
//! [`format_ranges`].

#![no_std]

extern crate alloc;

mod case;
mod join;
mod name;
mod ranges;
mod slug;
mod text;
mod transliteration;
mod units;

pub use case::*;
pub use join::*;
pub use name::*;
pub use ranges::*;
pub use slug::*;
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
