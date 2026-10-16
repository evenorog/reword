use unicode_segmentation::UnicodeSegmentation;

#[test]
fn truncate_preserves_complete_graphemes() {
    for (input, max_graphemes, expected) in [
        ("", 0, ""),
        ("Hello world", 0, ""),
        ("Hello world", 5, "Hello"),
        ("Hello world", 6, "Hello "),
        ("Hello", 5, "Hello"),
        ("Hello", usize::MAX, "Hello"),
        ("e\u{301}clair", 1, "e\u{301}"),
        ("a\u{308}\u{301}bc", 1, "a\u{308}\u{301}"),
        ("👩‍👩‍👧‍👦 family", 1, "👩‍👩‍👧‍👦"),
        ("👍🏽 ok", 1, "👍🏽"),
        ("🇳🇴🇸🇪", 1, "🇳🇴"),
        ("\r\nx", 1, "\r\n"),
        ("李小龍", 2, "李小"),
    ] {
        let output = reword::truncate(input, max_graphemes);
        assert_eq!(output, expected, "input: {input:?}");
        assert_eq!(output.as_ptr(), input.as_ptr());
        assert!(output.graphemes(true).count() <= max_graphemes);
    }
}
