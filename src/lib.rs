//! Provides some utility functions for human-readable formatting of words.

#![no_std]

extern crate alloc;

mod case;
mod join;
mod name;

pub use case::*;
pub use join::*;
pub use name::*;

use alloc::string::String;

fn fold(mut acc: String, w: &str, ch: char) -> String {
    if !acc.is_empty() {
        acc.push(ch);
    }
    acc.push_str(w);
    acc
}
