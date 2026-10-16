use alloc::string::String;
use deunicode::deunicode_char;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

/// Transliterates Unicode text to ASCII while preserving case and punctuation.
///
/// Normalizes the input to NFC, then uses [`deunicode::deunicode_char`] mappings.
/// ASCII input is returned unchanged, including whitespace and control
/// characters. Characters without a mapping become `"[?]"`; empty mappings are
/// omitted. Some transliterations introduce spaces or line breaks. A trailing
/// separator space from a mapping is omitted before whitespace or at the end.
///
/// The mappings are not language-specific and can be lossy; some symbols and
/// emoji become words. This function does not lowercase or filter the result.
/// Compose it with case conversion or other formatters as needed.
///
/// # Examples
/// ```
/// assert_eq!(reword::ascii("Crème brûlée!"), "Creme brulee!");
/// assert_eq!(reword::ascii("Æneid & Straße"), "AEneid & Strasse");
/// assert_eq!(reword::ascii("北亰"), "Bei Jing");
/// assert_eq!(reword::ascii("🦄☣"), "unicorn biohazard");
/// assert_eq!(reword::snake_case(reword::ascii("Crème brûlée")), "creme_brulee");
/// ```
pub fn ascii<T: AsRef<str>>(t: T) -> String {
    let input = t.as_ref();
    if input.is_ascii() {
        return input.into();
    }

    let mut output = String::with_capacity(input.len());
    let mut chars = input.nfc().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii() {
            output.push(c);
            continue;
        }

        let mapped = deunicode_char(c).unwrap_or("[?]");
        let trim_separator = mapped.len() > 1
            && mapped.ends_with(' ')
            && chars.peek().is_none_or(|next| next.is_whitespace());
        output.push_str(if trim_separator {
            &mapped[..mapped.len() - 1]
        } else {
            mapped
        });
    }
    output
}

/// Returns a borrowed prefix containing at most `max_graphemes` graphemes.
///
/// Counts extended Unicode grapheme clusters, including whitespace. Accented
/// letters, emoji sequences, and other clusters are kept intact. The limit does
/// not measure UTF-8 bytes or display width.
///
/// Returns the original string when it fits, or an empty prefix for a zero
/// limit. This function does not allocate or add an ellipsis.
///
/// # Examples
/// ```
/// assert_eq!(reword::truncate("Hello world", 5), "Hello");
/// assert_eq!(reword::truncate("e\u{301}clair", 1), "e\u{301}");
/// assert_eq!(reword::truncate("👩‍👩‍👧‍👦 family", 1), "👩‍👩‍👧‍👦");
/// assert_eq!(reword::truncate("hello", 0), "");
/// ```
pub fn truncate(text: &str, max_graphemes: usize) -> &str {
    if max_graphemes == 0 {
        return &text[..0];
    }
    let end = text
        .grapheme_indices(true)
        .nth(max_graphemes)
        .map_or(text.len(), |(index, _)| index);
    &text[..end]
}
