use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

#[test]
fn ascii_transliterates_text_and_preserves_case_and_punctuation() {
    for (input, expected) in [
        ("", ""),
        ("Crème brûlée!", "Creme brulee!"),
        ("Søren Weiß", "Soren Weiss"),
        ("Æneid & Straße", "AEneid & Strasse"),
        ("北亰", "Bei Jing"),
        ("北 亰", "Bei Jing"),
        ("北\t亰", "Bei\tJing"),
        ("北亰 ", "Bei Jing "),
        ("🦄☣", "unicorn biohazard"),
        ("1–3, 1.5 µs", "1-3, 1.5 us"),
        ("café\tMENU\n", "cafe\tMENU\n"),
        ("foo\u{10ffff}bar", "foo[?]bar"),
        ("\u{200d}", ""),
    ] {
        assert_eq!(reword::ascii(input), expected, "input: {input:?}");
    }
    assert_eq!(reword::ascii(String::from("Crème")), "Creme");
}

#[test]
fn ascii_preserves_all_ascii_characters_including_controls() {
    let input: String = (0..=127).map(char::from).collect();
    assert_eq!(reword::ascii(&input), input);
    assert_eq!(reword::ascii(format!("é{input}ø")), format!("e{input}o"));
}

#[test]
fn ascii_normalizes_canonically_equivalent_input() {
    for input in [
        "Crème brûlée",
        "Ångström",
        "İstanbul",
        "Καλημέρα κόσμε",
        "가",
    ] {
        let decomposed: String = input.nfd().collect();
        assert_eq!(reword::ascii(input), reword::ascii(decomposed));
    }
}

#[test]
fn ascii_composes_with_other_formatters() {
    assert_eq!(
        reword::snake_case(reword::ascii("Crème brûlée")),
        "creme_brulee"
    );
    assert_eq!(
        reword::ascii(reword::duration_short(core::time::Duration::from_nanos(
            1500
        ))),
        "1.5 us"
    );
    assert_eq!(reword::ascii(reword::format_ranges([1, 2, 3, 7])), "1-3, 7");
}

#[test]
fn transliterated_text_is_ascii_and_idempotent() {
    let parts = [
        "",
        "Hello, world!",
        "Crème",
        "ÆØß",
        "北亰",
        "🦄☣",
        "e\u{301}",
        "\u{301}",
        "\u{10ffff}",
        "\u{200d}",
        "\n\t",
        " ",
    ];
    for left in parts {
        for right in parts {
            let input = format!("{left}{right}");
            let output = reword::ascii(&input);
            assert!(output.is_ascii(), "input: {input:?}");
            assert_eq!(reword::ascii(&output), output, "input: {input:?}");
        }
    }
}

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
