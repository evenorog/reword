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
