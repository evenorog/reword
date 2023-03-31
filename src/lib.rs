//! Provides some utility functions for human-readable formatting of words.

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use unicode_segmentation::UnicodeSegmentation;

fn pat(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_whitespace()
}

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

fn fold(mut acc: String, w: &str, ch: char) -> String {
    if !acc.is_empty() {
        acc.push(ch);
    }
    acc.push_str(w);
    acc
}

fn fold_camel_case(mut acc: String, w: String) -> String {
    let start_is_num = w.chars().next().map_or(false, char::is_numeric);
    let end_is_num = acc.chars().last().map_or(false, char::is_numeric);
    // Split with _ if two word boundaries are numeric.
    if start_is_num && end_is_num {
        acc.push('_');
    }
    acc.push_str(&w);
    acc
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
    let mut s = String::with_capacity(upper.unwrap_or(lower) * 16);
    s.push_str(first.as_ref());
    let Some(mut next) = iter.next() else {
        return s;
    };

    for peek in iter {
        s.push_str(mid_sep);
        s.push_str(next.as_ref());
        next = peek;
    }

    s.push_str(end_sep);
    s.push_str(next.as_ref());
    s
}

#[cfg(test)]
mod tests {
    const S: &str = "(Even), Olsson&Rogstadkjærnet?";

    #[test]
    fn name() {
        assert_eq!(crate::name(S), "Even Olsson Rogstadkjærnet");
        assert_eq!(crate::name_with_limit(S, 25), "Even O Rogstadkjærnet");
        assert_eq!(crate::name_with_limit(S, 12), "Even O R");
        assert_eq!(crate::name_with_limit(S, 7), "E O R");
        assert_eq!(crate::name_with_limit(S, 4), "EOR");
        assert_eq!(crate::name_with_limit(S, 2), "EO");
        assert_eq!(crate::name_with_limit(S, 1), "E");
        assert_eq!(crate::name_with_limit(S, 0), "");
    }

    #[test]
    fn username() {
        assert_eq!(crate::username(S), "evenolssonrogstadkjærnet");
        assert_eq!(crate::username_with_limit(S, 25), "evenorogstadkjærnet");
        assert_eq!(crate::username_with_limit(S, 12), "evenor");
        assert_eq!(crate::username_with_limit(S, 7), "eor");
        assert_eq!(crate::username_with_limit(S, 4), "eor");
        assert_eq!(crate::username_with_limit(S, 2), "eo");
        assert_eq!(crate::username_with_limit(S, 1), "e");
        assert_eq!(crate::username_with_limit(S, 0), "");
    }

    #[test]
    fn join() {
        assert_eq!(crate::or_join::<&[&str]>(&[]), "");
        assert_eq!(crate::or_join::<&[&str]>(&["a"]), "a");
        assert_eq!(crate::or_join::<&[&str]>(&["a", "b"]), "a or b");
        assert_eq!(crate::or_join::<&[&str]>(&["a", "b", "c"]), "a, b or c");
        assert_eq!(
            crate::or_join::<&[&str]>(&["a", "b", "c", "d", "e"]),
            "a, b, c, d or e"
        );
        assert_eq!(crate::and_join::<&[&str]>(&[]), "");
        assert_eq!(crate::and_join::<&[&str]>(&["a"]), "a");
        assert_eq!(crate::and_join::<&[&str]>(&["a", "b"]), "a and b");
        assert_eq!(crate::and_join::<&[&str]>(&["a", "b", "c"]), "a, b and c");
        assert_eq!(
            crate::and_join::<&[&str]>(&["a", "b", "c", "d", "e"]),
            "a, b, c, d and e"
        );
    }

    #[test]
    fn camel_case() {
        assert_eq!(crate::camel_case("AGPL_3_0_or_later"), "agpl3_0OrLater");
        assert_eq!(crate::camel_case("MIT"), "mit");
    }

    #[test]
    fn pascal_case() {
        assert_eq!(crate::pascal_case("AGPL_3_0_or_later"), "Agpl3_0OrLater");
        assert_eq!(crate::pascal_case("MIT"), "Mit");
    }

    #[test]
    fn snake_case() {
        assert_eq!(crate::snake_case("AGPL_3_0_or_later"), "agpl_3_0_or_later");
        assert_eq!(crate::snake_case("MIT"), "mit");
    }

    #[test]
    fn screaming_snake_case() {
        assert_eq!(
            crate::screaming_snake_case("AGPL_3_0_or_later"),
            "AGPL_3_0_OR_LATER"
        );
        assert_eq!(crate::screaming_snake_case("MIT"), "MIT");
    }

    #[test]
    fn kebab_case() {
        assert_eq!(crate::kebab_case("AGPL_3_0_or_later"), "agpl-3-0-or-later");
        assert_eq!(crate::kebab_case("MIT"), "mit");
    }

    #[test]
    fn screaming_kebab_case() {
        assert_eq!(
            crate::screaming_kebab_case("AGPL_3_0_or_later"),
            "AGPL-3-0-OR-LATER"
        );
        assert_eq!(crate::screaming_kebab_case("MIT"), "MIT");
    }
}
