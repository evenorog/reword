use unicode_normalization::UnicodeNormalization;

#[test]
fn slugs_normalize_words_and_separators() {
    for (input, expected) in [
        ("", ""),
        (" ---!? ", ""),
        ("My First Article!", "my-first-article"),
        ("  Hello___WORLD / 42!  ", "hello-world-42"),
        ("foo\t\r\nbar\u{a0}baz\u{2003}qux", "foo-bar-baz-qux"),
        ("already-a-slug", "already-a-slug"),
        ("helloWorld", "helloworld"),
        ("Crème brûlée", "creme-brulee"),
        ("Rød grøt med fløte", "rod-grot-med-flote"),
        ("Æneid & Straße", "aeneid-strasse"),
        ("Æ Ø Å ß Ł Đ Þ", "ae-o-a-ss-l-d-th"),
        ("北亰", "bei-jing"),
        ("Rust 🦀", "rust-crab"),
        ("foo\u{10ffff}bar", "foo-bar"),
        ("foo\u{200d}bar", "foobar"),
    ] {
        assert_eq!(reword::slug_ascii(input), expected, "input: {input:?}");
    }
    assert_eq!(
        reword::slug_ascii(String::from("An owned title")),
        "an-owned-title"
    );
}

#[test]
fn canonically_equivalent_titles_produce_the_same_slug() {
    for input in ["Crème brûlée", "Ångström", "İstanbul", "Καλημέρα κόσμε"] {
        let decomposed: String = input.nfd().collect();
        assert_eq!(reword::slug_ascii(input), reword::slug_ascii(decomposed));
    }
}

#[test]
fn slugs_have_clean_ascii_boundaries_and_are_idempotent() {
    let fragments = [
        "",
        "---",
        "helloWorld",
        "Hello, world!",
        "123",
        "Crème",
        "ÆØß",
        "北亰",
        "🦄☣",
        "e\u{301}",
        "\u{301}",
        "\u{10ffff}",
        "\u{200d}",
        "\n\t",
    ];
    for left in fragments {
        for right in fragments {
            let input = format!("{left}{right}");
            let slug = reword::slug_ascii(&input);
            assert!(
                slug.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            );
            assert!(!slug.starts_with('-'), "input: {input:?}");
            assert!(!slug.ends_with('-'), "input: {input:?}");
            assert!(!slug.contains("--"), "input: {input:?}");
            assert_eq!(reword::slug_ascii(&slug), slug);
        }
    }
}
