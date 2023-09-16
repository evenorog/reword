use crate::{fold, name, name_with_limit};
use alloc::string::String;

fn pat(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_whitespace()
}

/// Formats the input string as a kebab case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::kebab_case("Even Olsson Rogstadkjærnet"), "even-olsson-rogstadkjærnet");
/// ```
pub fn kebab_case<T: AsRef<str>>(t: T) -> String {
    name(t)
        .split(pat)
        .filter(|t| !t.is_empty())
        .map(str::to_lowercase)
        .fold(String::new(), fold_kebab_case)
}

/// Formats the input string as a kebab case name and limits the length of the name.
///
/// # Examples
/// ```
/// assert_eq!(reword::kebab_case_with_limit("Even Olsson Rogstadkjærnet", 25), "even-o-rogstadkjærnet");
/// ```
pub fn kebab_case_with_limit<T: AsRef<str>>(t: T, limit: usize) -> String {
    name_with_limit(t, limit)
        .split(pat)
        .filter(|t| !t.is_empty())
        .map(str::to_lowercase)
        .fold(String::new(), fold_kebab_case)
}

/// Formats the input string as a screaming kebab case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::screaming_kebab_case("Even Olsson Rogstadkjærnet"), "EVEN-OLSSON-ROGSTADKJÆRNET");
/// ```
pub fn screaming_kebab_case<T: AsRef<str>>(t: T) -> String {
    name(t)
        .split(pat)
        .filter(|t| !t.is_empty())
        .map(str::to_uppercase)
        .fold(String::new(), fold_kebab_case)
}

/// Formats the input string as a screaming kebab case name and limits the length of the name.
///
/// # Examples
/// ```
/// assert_eq!(reword::screaming_kebab_case_with_limit("Even Olsson Rogstadkjærnet", 25), "EVEN-O-ROGSTADKJÆRNET");
/// ```
pub fn screaming_kebab_case_with_limit<T: AsRef<str>>(t: T, limit: usize) -> String {
    name_with_limit(t, limit)
        .split(pat)
        .filter(|t| !t.is_empty())
        .map(str::to_uppercase)
        .fold(String::new(), fold_kebab_case)
}

/// Formats the input string as a snake case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::snake_case("Even Olsson Rogstadkjærnet"), "even_olsson_rogstadkjærnet");
/// ```
pub fn snake_case<T: AsRef<str>>(t: T) -> String {
    name(t)
        .split(pat)
        .filter(|t| !t.is_empty())
        .map(str::to_lowercase)
        .fold(String::new(), fold_snake_case)
}

/// Formats the input string as a snake case name and limits the length of the name.
///
/// # Examples
/// ```
/// assert_eq!(reword::snake_case_with_limit("Even Olsson Rogstadkjærnet", 25), "even_o_rogstadkjærnet");
/// ```
pub fn snake_case_with_limit<T: AsRef<str>>(t: T, limit: usize) -> String {
    name_with_limit(t, limit)
        .split(pat)
        .filter(|t| !t.is_empty())
        .map(str::to_lowercase)
        .fold(String::new(), fold_snake_case)
}

/// Formats the input string as a screaming snake case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::screaming_snake_case("Even Olsson Rogstadkjærnet"), "EVEN_OLSSON_ROGSTADKJÆRNET");
/// ```
pub fn screaming_snake_case<T: AsRef<str>>(t: T) -> String {
    name(t)
        .split(pat)
        .filter(|t| !t.is_empty())
        .map(str::to_uppercase)
        .fold(String::new(), fold_snake_case)
}

/// Formats the input string as a screaming snake case name and limits the length of the name.
///
/// # Examples
/// ```
/// assert_eq!(reword::screaming_snake_case_with_limit("Even Olsson Rogstadkjærnet", 25), "EVEN_O_ROGSTADKJÆRNET");
/// ```
pub fn screaming_snake_case_with_limit<T: AsRef<str>>(t: T, limit: usize) -> String {
    name_with_limit(t, limit)
        .split(pat)
        .filter(|t| !t.is_empty())
        .map(str::to_uppercase)
        .fold(String::new(), fold_snake_case)
}

/// Formats the input string as a camel case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::camel_case("Even Olsson Rogstadkjærnet"), "evenOlssonRogstadkjærnet");
/// ```
pub fn camel_case<T: AsRef<str>>(t: T) -> String {
    name(t)
        .split(pat)
        .filter(|t| !t.is_empty())
        .enumerate()
        .map(|(i, word)| to_camel_case(word, i != 0))
        .fold(String::new(), fold_camel_case)
}

/// Formats the input string as a camel case name with a limit on the length.
///
/// # Examples
/// ```
/// assert_eq!(reword::camel_case_with_limit("Even Olsson Rogstadkjærnet", 25), "evenORogstadkjærnet");
/// ```
pub fn camel_case_with_limit<T: AsRef<str>>(t: T, limit: usize) -> String {
    name_with_limit(t, limit)
        .split(pat)
        .filter(|t| !t.is_empty())
        .enumerate()
        .map(|(i, word)| to_camel_case(word, i != 0))
        .fold(String::new(), fold_camel_case)
}

/// Formats the input string as a pascal case name.
///
/// # Examples
/// ```
/// assert_eq!(reword::pascal_case("Even Olsson Rogstadkjærnet"), "EvenOlssonRogstadkjærnet");
/// ```
pub fn pascal_case<T: AsRef<str>>(t: T) -> String {
    name(t)
        .split(pat)
        .filter(|t| !t.is_empty())
        .map(|word| to_camel_case(word, true))
        .fold(String::new(), fold_camel_case)
}

/// Formats the input string as a pascal case name with a limit on the length.
///
/// # Examples
/// ```
/// assert_eq!(reword::pascal_case_with_limit("Even Olsson Rogstadkjærnet", 25), "EvenORogstadkjærnet");
/// ```
pub fn pascal_case_with_limit<T: AsRef<str>>(t: T, limit: usize) -> String {
    name_with_limit(t, limit)
        .split(pat)
        .filter(|t| !t.is_empty())
        .map(|word| to_camel_case(word, true))
        .fold(String::new(), fold_camel_case)
}

fn to_camel_case(word: &str, mut upper: bool) -> String {
    let mut cc = String::new();
    let mut prev_is_lowercase = false;

    for c in word.chars() {
        if upper {
            cc.extend(c.to_uppercase());
        } else {
            cc.extend(c.to_lowercase());
        }

        upper = prev_is_lowercase && c.is_uppercase();
        prev_is_lowercase = c.is_lowercase();
    }

    cc
}

fn fold_kebab_case(acc: String, w: String) -> String {
    fold(acc, &w, '-')
}

fn fold_snake_case(acc: String, w: String) -> String {
    fold(acc, &w, '_')
}

fn fold_camel_case(mut acc: String, w: String) -> String {
    let start_is_num = matches!(w.chars().next(), Some(c) if c.is_numeric());
    let end_is_num = matches!(acc.chars().last(), Some(c) if c.is_numeric());
    // Split with _ if two word boundaries are numeric.
    if start_is_num && end_is_num {
        acc.push('_');
    }
    acc.push_str(&w);
    acc
}
