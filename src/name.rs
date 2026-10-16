use crate::{fold, transliteration::transliterated_ascii};
use alloc::string::String;
use alloc::vec::Vec;
use unicode_normalization::UnicodeNormalization;
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

/// Abbreviates a name to at most `max_graphemes` extended grapheme clusters.
///
/// Extracts words using the same rules as [`name`]. Words after the first are
/// shortened to their first grapheme, from left to right, followed by the first
/// word if necessary. If the initials still do not fit with spaces, the spaces
/// are removed, then trailing initials are discarded as needed.
///
/// Spaces count toward the limit. This limits grapheme clusters, not UTF-8 bytes
/// or display width. A zero limit or an input without words returns an empty
/// string. Formatting the result again may change its length.
///
/// # Examples
/// ```
/// let t = "(Even),Olsson&Rogstadkjærnet?";
/// assert_eq!(reword::abbreviate_name(t, 4), "EOR");
/// assert_eq!(reword::abbreviate_name(t, 25), "Even O Rogstadkjærnet");
///
/// let short_name = reword::abbreviate_name(t, 25);
/// assert_eq!(reword::snake_case(&short_name), "even_o_rogstadkjærnet");
/// ```
pub fn abbreviate_name<T: AsRef<str>>(t: T, max_graphemes: usize) -> String {
    if max_graphemes == 0 {
        return String::new();
    }

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
        if count <= max_graphemes {
            break;
        }
        count -= c - 1;
        *w = w.graphemes(true).next().unwrap();
    }

    // Checks if the first name also needs to be shortened.
    if let Some((w, c)) = head
        && count > max_graphemes
    {
        count -= c - 1;
        *w = w.graphemes(true).next().unwrap();
    }

    if count <= max_graphemes {
        name.join(" ")
    } else if (count - spaces) <= max_graphemes {
        name.concat()
    } else {
        name[..max_graphemes].concat()
    }
}

/// Creates a lowercase Unicode username candidate from the provided string.
///
/// Applies locale-independent Unicode lowercasing and NFC normalization, then
/// removes characters for which [`char::is_alphanumeric`] returns false. The
/// result is normalized to NFC again after filtering.
///
/// Accented letters and non-Latin scripts are retained; the result is not
/// transliterated to ASCII. Spaces, punctuation, and non-alphanumeric combining
/// marks that could not be composed are removed. The result can be empty, has no
/// length limit, and is not checked for uniqueness or service-specific rules.
/// Use [`username_ascii`] to transliterate letters to ASCII instead.
///
/// # Examples
/// ```
/// assert_eq!(reword::username("Even Olsson Rogstadkjærnet"), "evenolssonrogstadkjærnet");
/// assert_eq!(reword::username("Jose\u{301}"), "josé");
/// assert_eq!(reword::username("İ"), "i");
/// assert_eq!(reword::username("🦀!"), "");
/// ```
pub fn username<T: AsRef<str>>(t: T) -> String {
    let input = t.as_ref();
    if input.is_ascii() {
        let mut username = input.to_ascii_lowercase();
        username.retain(|c| c.is_ascii_alphanumeric());
        return username;
    }

    input
        .to_lowercase()
        .nfc()
        .filter(|c| c.is_alphanumeric())
        .nfc()
        .collect()
}

/// Creates a lowercase ASCII username candidate by transliterating Unicode.
///
/// Normalizes the input to NFC, then transliterates each character using
/// [`deunicode::deunicode_char`]. Retains only ASCII letters and digits, with
/// letters converted to lowercase. Spaces, punctuation, and characters without
/// a mapping are removed. The mappings are not language-specific; some symbols
/// and emoji map to words, which are retained.
///
/// The result can be empty, has no length limit, and is not checked for
/// uniqueness or service-specific rules. Use [`username`] to preserve Unicode
/// letters instead.
///
/// # Examples
/// ```
/// assert_eq!(reword::username_ascii("Søren Weiß"), "sorenweiss");
/// assert_eq!(reword::username_ascii("Jose\u{301}"), "jose");
/// assert_eq!(reword::username_ascii("(O'Connor)_99"), "oconnor99");
/// assert_eq!(reword::username_ascii("🦄!"), "unicorn");
/// ```
pub fn username_ascii<T: AsRef<str>>(t: T) -> String {
    let input = t.as_ref();
    let mut output = String::with_capacity(input.len());
    output.extend(
        transliterated_ascii(input)
            .filter(u8::is_ascii_alphanumeric)
            .map(|byte| char::from(byte.to_ascii_lowercase())),
    );
    output
}
