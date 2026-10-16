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

#[test]
fn configurable_joins() {
    use reword::{JoinOptions, join_with};

    let names = ["Alice", "Bob", "Charlie"];
    assert_eq!(
        join_with(names, JoinOptions::default()),
        "Alice, Bob and Charlie"
    );
    let serial = JoinOptions {
        serial_comma: true,
        ..JoinOptions::default()
    };
    assert_eq!(join_with(names, serial), "Alice, Bob, and Charlie");
    assert_eq!(join_with(["Alice", "Bob"], serial), "Alice and Bob");
    assert_eq!(join_with(["Alice"], serial), "Alice");
    assert_eq!(join_with(core::iter::empty::<&str>(), serial), "");

    assert_eq!(
        join_with(
            names,
            JoinOptions {
                conjunction: "or",
                serial_comma: true
            }
        ),
        "Alice, Bob, or Charlie"
    );
    let conjunction = String::from("og");
    assert_eq!(
        join_with(
            names,
            JoinOptions {
                conjunction: &conjunction,
                serial_comma: false
            }
        ),
        "Alice, Bob og Charlie"
    );
    for serial_comma in [false, true] {
        let comma_only = JoinOptions {
            conjunction: "",
            serial_comma,
        };
        assert_eq!(join_with(names, comma_only), "Alice, Bob, Charlie");
        assert_eq!(join_with(["Alice", "Bob"], comma_only), "Alice, Bob");
    }
}

#[test]
fn configurable_joins_accept_one_shot_owned_items() {
    let mut next = 0;
    let items = core::iter::from_fn(|| {
        next += 1;
        (next <= 3).then(|| next.to_string())
    });
    assert_eq!(
        reword::join_with(items, reword::JoinOptions::default()),
        "1, 2 and 3"
    );
    assert_eq!(reword::and_join(["", "b", ""]), ", b and ");
}

#[test]
fn joins_do_not_reserve_an_iterators_loose_upper_bound() {
    struct LooseHint<I>(I);
    impl<I: Iterator> Iterator for LooseHint<I> {
        type Item = I::Item;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(usize::MAX))
        }
    }

    assert_eq!(
        reword::and_join(LooseHint(["a", "b"].into_iter())),
        "a and b"
    );
    assert_eq!(
        reword::join_with(
            LooseHint(["a", "b", "c"].into_iter()),
            reword::JoinOptions {
                serial_comma: true,
                ..reword::JoinOptions::default()
            }
        ),
        "a, b, and c"
    );
}
