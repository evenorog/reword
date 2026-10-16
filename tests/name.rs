use unicode_normalization::{UnicodeNormalization, is_nfc};
use unicode_segmentation::UnicodeSegmentation;

const S: &str = "(Even), Olsson&Rogstadkjærnet?";

#[test]
fn name() {
    assert_eq!(reword::name(S), "Even Olsson Rogstadkjærnet");
}

#[test]
fn abbreviate_name() {
    assert_eq!(reword::abbreviate_name(S, 25), "Even O Rogstadkjærnet");
    assert_eq!(reword::abbreviate_name(S, 12), "Even O R");
    assert_eq!(reword::abbreviate_name(S, 7), "E O R");
    assert_eq!(reword::abbreviate_name(S, 4), "EOR");
    assert_eq!(reword::abbreviate_name(S, 2), "EO");
    assert_eq!(reword::abbreviate_name(S, 1), "E");
    assert_eq!(reword::abbreviate_name(S, 0), "");
}

#[test]
fn abbreviation_preserves_graphemes() {
    assert_eq!(reword::abbreviate_name("Émile", 1), "É");
    assert_eq!(reword::abbreviate_name("e\u{301}mile", 1), "e\u{301}");
    assert_eq!(
        reword::abbreviate_name("a\u{308}\u{301}bc", 1),
        "a\u{308}\u{301}"
    );
}

#[test]
fn abbreviation_respects_its_grapheme_limit() {
    for input in [
        "",
        "! 🦀",
        "One",
        "A B C D",
        S,
        "Émile José",
        "e\u{301}mile Jose\u{301}",
        "a\u{308}\u{301}bc def",
        "李 小龍",
        "क्ष शर्मा",
        "İ Σ ﬃ ß",
    ] {
        for max_graphemes in 0..=32 {
            let output = reword::abbreviate_name(input, max_graphemes);
            assert!(
                output.graphemes(true).count() <= max_graphemes,
                "input: {input:?}, max_graphemes: {max_graphemes}, output: {output:?}"
            );
        }
        assert_eq!(
            reword::abbreviate_name(input, usize::MAX),
            reword::name(input)
        );
    }
}

#[test]
fn abbreviation_composes_with_other_formatters() {
    let short_name = reword::abbreviate_name(S, 25);
    assert_eq!(reword::snake_case(&short_name), "even_o_rogstadkjærnet");
    assert_eq!(reword::camel_case(&short_name), "evenORogstadkjærnet");
    assert_eq!(reword::username(&short_name), "evenorogstadkjærnet");
}

#[test]
fn username() {
    assert_eq!(reword::username(S), "evenolssonrogstadkjærnet");
    for (input, expected) in [
        ("", ""),
        (" -_ 🦀!", ""),
        ("\u{301}", ""),
        ("(O'Connor)_99", "oconnor99"),
        ("USER\n123", "user123"),
        ("İ", "i"),
        ("İSTANBUL", "istanbul"),
        ("José", "josé"),
        ("Jose\u{301}", "josé"),
        ("A\u{30a}SE", "åse"),
        ("Straße", "straße"),
        ("ΟΣ", "ος"),
        ("用户１２", "用户１２"),
        ("ﬁ", "ﬁ"),
        // Filtering can bring together letters and alphabetic combining marks.
        ("α-\u{345}", "ᾳ"),
    ] {
        assert_eq!(reword::username(input), expected, "input: {input:?}");
    }
}

#[test]
fn username_contract() {
    let parts = [
        "",
        "USER",
        "Jane Doe",
        "José",
        "Jose\u{301}",
        "İ",
        "ΟΣ",
        "ß",
        "ﬁ",
        "李",
        "用户１２",
        "क्ष",
        "α",
        "\u{345}",
        "\u{301}",
        "123",
        "_",
        "-",
        " ",
        "\n",
        "🦀",
    ];
    for left in parts {
        for right in parts {
            let input = format!("{left}{right}");
            let output = reword::username(&input);
            assert!(
                output.chars().all(char::is_alphanumeric),
                "input: {input:?}"
            );
            assert!(is_nfc(&output), "input: {input:?}");
            assert_eq!(reword::username(&output), output, "input: {input:?}");
        }
    }
}

#[test]
fn username_ascii() {
    assert_eq!(reword::username_ascii(S), "evenolssonrogstadkjaernet");
    for (input, expected) in [
        ("", ""),
        (" -_ !", ""),
        ("\u{301}", ""),
        ("(O'Connor)_99", "oconnor99"),
        ("USER\n123", "user123"),
        ("helloWorld", "helloworld"),
        ("İSTANBUL", "istanbul"),
        ("José", "jose"),
        ("Jose\u{301}", "jose"),
        ("Søren Weiß", "sorenweiss"),
        ("Æneid", "aeneid"),
        ("ﬁ", "fi"),
        ("１２３", "123"),
        ("北亰", "beijing"),
        ("🦄!", "unicorn"),
        ("foo\u{10ffff}bar", "foobar"),
        ("foo\u{200d}bar", "foobar"),
    ] {
        assert_eq!(reword::username_ascii(input), expected, "input: {input:?}");
    }
    assert_eq!(reword::username_ascii(String::from("Jane Doe")), "janedoe");
}

#[test]
fn ascii_usernames_normalize_canonically_equivalent_input() {
    for input in ["Crème brûlée", "Ångström", "İstanbul", "Καλημέρα κόσμε"] {
        let decomposed: String = input.nfd().collect();
        assert_eq!(
            reword::username_ascii(input),
            reword::username_ascii(decomposed)
        );
    }
}

#[test]
fn ascii_usernames_match_unicode_usernames_for_ascii_input() {
    let input: String = (0..=127).map(char::from).collect();
    assert_eq!(reword::username_ascii(&input), reword::username(&input));
}

#[test]
fn ascii_usernames_are_lowercase_alphanumeric_and_idempotent() {
    let parts = [
        "",
        "USER",
        "Jane Doe",
        "José",
        "Jose\u{301}",
        "ÆØß",
        "北亰",
        "İ",
        "ΟΣ",
        "ﬁ",
        "123",
        "_",
        "-",
        " ",
        "\n",
        "🦄☣",
        "\u{301}",
        "\u{10ffff}",
        "\u{200d}",
    ];
    for left in parts {
        for right in parts {
            let input = format!("{left}{right}");
            let output = reword::username_ascii(&input);
            assert!(
                output
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit()),
                "input: {input:?}"
            );
            assert_eq!(reword::username_ascii(&output), output, "input: {input:?}");
        }
    }
}
