use unicode_segmentation::UnicodeSegmentation;

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
