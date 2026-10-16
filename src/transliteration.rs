use deunicode::deunicode_char;
use unicode_normalization::UnicodeNormalization;

pub(crate) fn transliterated_ascii(input: &str) -> impl Iterator<Item = u8> + '_ {
    input.nfc().flat_map(|c| {
        // Preserve word boundaries even when the mapping drops whitespace or
        // has no transliteration. Consumers decide how to handle separators.
        let mapped = if c.is_whitespace() {
            " "
        } else {
            deunicode_char(c).unwrap_or(" ")
        };
        mapped.bytes()
    })
}
