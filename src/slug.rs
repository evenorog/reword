use crate::transliteration::transliterated_ascii;
use alloc::string::String;

/// Creates a lowercase ASCII slug with hyphen-separated words.
///
/// Normalizes the input to NFC, then transliterates each character using
/// [`deunicode::deunicode_char`]. The mappings are not language-specific; some
/// symbols and emoji map to words. ASCII letters and digits are retained, while
/// punctuation and whitespace collapse to a single separating hyphen.
///
/// Characters without a mapping also separate words. Empty mappings, including
/// many combining marks, are ignored. Leading and trailing hyphens are omitted.
/// Existing camel-case boundaries do not introduce hyphens. The result can be
/// empty and has no length or uniqueness guarantee.
///
/// # Examples
/// ```
/// assert_eq!(reword::slug_ascii("My First Article!"), "my-first-article");
/// assert_eq!(reword::slug_ascii("Crème brûlée"), "creme-brulee");
/// assert_eq!(reword::slug_ascii("Rød grøt med fløte"), "rod-grot-med-flote");
/// assert_eq!(reword::slug_ascii("Æneid & Straße"), "aeneid-strasse");
/// ```
pub fn slug_ascii<T: AsRef<str>>(t: T) -> String {
    let input = t.as_ref();
    let mut output = String::with_capacity(input.len());
    let mut separator = false;

    for byte in transliterated_ascii(input) {
        if byte.is_ascii_alphanumeric() {
            if separator && !output.is_empty() {
                output.push('-');
            }
            output.push(char::from(byte.to_ascii_lowercase()));
            separator = false;
        } else {
            separator = true;
        }
    }
    output
}
