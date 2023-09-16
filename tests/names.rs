const S: &str = "(Even), Olsson&Rogstadkjærnet?";

#[test]
fn name() {
    assert_eq!(reword::name(S), "Even Olsson Rogstadkjærnet");
    assert_eq!(reword::name_with_limit(S, 25), "Even O Rogstadkjærnet");
    assert_eq!(reword::name_with_limit(S, 12), "Even O R");
    assert_eq!(reword::name_with_limit(S, 7), "E O R");
    assert_eq!(reword::name_with_limit(S, 4), "EOR");
    assert_eq!(reword::name_with_limit(S, 2), "EO");
    assert_eq!(reword::name_with_limit(S, 1), "E");
    assert_eq!(reword::name_with_limit(S, 0), "");
}

#[test]
fn username() {
    assert_eq!(reword::username(S), "evenolssonrogstadkjærnet");
    assert_eq!(reword::username_with_limit(S, 25), "evenorogstadkjærnet");
    assert_eq!(reword::username_with_limit(S, 12), "evenor");
    assert_eq!(reword::username_with_limit(S, 7), "eor");
    assert_eq!(reword::username_with_limit(S, 4), "eor");
    assert_eq!(reword::username_with_limit(S, 2), "eo");
    assert_eq!(reword::username_with_limit(S, 1), "e");
    assert_eq!(reword::username_with_limit(S, 0), "");
}

#[test]
fn join() {
    assert_eq!(reword::or_join::<&[&str]>(&[]), "");
    assert_eq!(reword::or_join::<&[&str]>(&["a"]), "a");
    assert_eq!(reword::or_join::<&[&str]>(&["a", "b"]), "a or b");
    assert_eq!(reword::or_join::<&[&str]>(&["a", "b", "c"]), "a, b or c");
    assert_eq!(
        reword::or_join::<&[&str]>(&["a", "b", "c", "d", "e"]),
        "a, b, c, d or e"
    );
    assert_eq!(reword::and_join::<&[&str]>(&[]), "");
    assert_eq!(reword::and_join::<&[&str]>(&["a"]), "a");
    assert_eq!(reword::and_join::<&[&str]>(&["a", "b"]), "a and b");
    assert_eq!(reword::and_join::<&[&str]>(&["a", "b", "c"]), "a, b and c");
    assert_eq!(
        reword::and_join::<&[&str]>(&["a", "b", "c", "d", "e"]),
        "a, b, c, d and e"
    );
}
