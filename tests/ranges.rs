#[test]
fn ranges_compact_consecutive_values_and_adjacent_duplicates() {
    for (input, expected) in [
        (&[][..], ""),
        (&[1][..], "1"),
        (&[1, 2][..], "1–2"),
        (&[1, 3][..], "1, 3"),
        (&[1, 2, 3, 7, 9, 10][..], "1–3, 7, 9–10"),
        (&[1, 1, 2, 2, 3, 7, 7][..], "1–3, 7"),
        (&[-3, -2, -1, 0, 1, 2][..], "-3–2"),
        (&[-3, -2, -1, 1, 2][..], "-3–-1, 1–2"),
        (&[3, 1, 2][..], "3, 1–2"),
        (&[3, 2, 1][..], "3, 2, 1"),
        (&[1, 2, 1, 2][..], "1–2, 1–2"),
    ] {
        assert_eq!(reword::format_ranges(input.iter().copied()), expected);
    }
    assert_eq!(reword::format_ranges(core::iter::empty::<i32>()), "");
}

#[test]
fn ranges_accept_one_shot_iterators() {
    let mut next = 0;
    let values = core::iter::from_fn(|| {
        next += 1;
        (next <= 4).then_some(next)
    });
    assert_eq!(reword::format_ranges(values), "1–4");
}

#[test]
fn ranges_support_all_primitive_integer_types_without_overflow() {
    macro_rules! check {
        ($($ty:ty),+ $(,)?) => {
            $(
                let min = <$ty>::MIN;
                let max = <$ty>::MAX;
                assert_eq!(
                    reword::format_ranges([min, min + 1, max - 1, max, max]),
                    format!("{min}–{}, {}–{max}", min + 1, max - 1),
                    "type: {}", stringify!($ty)
                );
                assert_eq!(
                    reword::format_ranges([max, min]),
                    format!("{max}, {min}"),
                    "type: {}", stringify!($ty)
                );
            )+
        };
    }
    check!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );
}

#[test]
fn expanding_ranges_recovers_input_order_without_adjacent_duplicates() {
    for seed in 0..100 {
        let input: Vec<i64> = (0..100)
            .map(|i| ((i * i + seed * i + seed) % 41) - 20)
            .collect();
        let formatted = reword::format_ranges(input.iter().copied());
        let mut expanded = Vec::new();
        for part in formatted.split(", ") {
            if let Some((start, end)) = part.split_once('–') {
                let start: i64 = start.parse().unwrap();
                let end: i64 = end.parse().unwrap();
                assert!(start < end);
                expanded.extend(start..=end);
            } else {
                expanded.push(part.parse().unwrap());
            }
        }
        let mut expected = input;
        expected.dedup();
        assert_eq!(expanded, expected);
    }
}
