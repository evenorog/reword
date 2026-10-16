use reword::units;

#[test]
fn binary_bytes() {
    for (input, expected) in [
        (0, "0 B"),
        (1, "1 B"),
        (1023, "1023 B"),
        (1024, "1.0 KiB"),
        (1536, "1.5 KiB"),
        (1_048_524, "1023.9 KiB"),
        (1_048_525, "1.0 MiB"),
        (u64::MAX, "16.0 EiB"),
    ] {
        assert_eq!(units::bytes(input), expected, "input: {input}");
    }
    for (power, suffix) in ["KiB", "MiB", "GiB", "TiB", "PiB", "EiB"]
        .into_iter()
        .enumerate()
    {
        let input = 1024_u64.pow(power as u32 + 1);
        assert_eq!(units::bytes(input), format!("1.0 {suffix}"));
    }
}

#[test]
fn decimal_bytes() {
    for (input, expected) in [
        (0, "0 B"),
        (999, "999 B"),
        (1000, "1.0 kB"),
        (1050, "1.1 kB"),
        (1500, "1.5 kB"),
        (999_949, "999.9 kB"),
        (999_950, "1.0 MB"),
        (u64::MAX, "18.4 EB"),
    ] {
        assert_eq!(units::bytes_decimal(input), expected, "input: {input}");
    }
    for (power, suffix) in ["kB", "MB", "GB", "TB", "PB", "EB"].into_iter().enumerate() {
        let input = 1000_u64.pow(power as u32 + 1);
        assert_eq!(units::bytes_decimal(input), format!("1.0 {suffix}"));
    }
}

#[test]
fn byte_rounding_stays_within_half_a_displayed_tenth() {
    type ByteFormat = (fn(u64) -> String, u128, [&'static str; 7]);
    let formats: [ByteFormat; 2] = [
        (
            units::bytes,
            1024,
            ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"],
        ),
        (
            units::bytes_decimal,
            1000,
            ["B", "kB", "MB", "GB", "TB", "PB", "EB"],
        ),
    ];
    let mut inputs = vec![0, 1, 999, 1000, 1023, 1024, u64::MAX];
    let mut seed = 42_u64;
    for _ in 0..200 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        inputs.push(seed);
    }
    for (formatter, base, suffixes) in formats {
        for &input in &inputs {
            let output = formatter(input);
            let (value, suffix) = output.split_once(' ').unwrap();
            let unit = suffixes.iter().position(|&unit| unit == suffix).unwrap();
            if unit == 0 {
                assert_eq!(value.parse::<u64>().unwrap(), input);
                continue;
            }
            let (whole, fractional) = value.split_once('.').unwrap();
            assert_eq!(fractional.len(), 1);
            let tenths = whole.parse::<u128>().unwrap() * 10 + fractional.parse::<u128>().unwrap();
            assert!(tenths < base * 10, "input: {input}, output: {output}");
            let multiplier = base.pow(unit as u32);
            let error = (tenths * multiplier).abs_diff(u128::from(input) * 10);
            assert!(error <= multiplier / 2, "input: {input}, output: {output}");
        }
    }
}

#[test]
fn grouped_numbers_preserve_integer_precision() {
    assert_eq!(units::number(0), "0");
    assert_eq!(units::number(1234), "1,234");
    assert_eq!(units::number(-1234), "-1,234");
    assert_eq!(
        units::number(9_007_199_254_740_993_u64),
        "9,007,199,254,740,993"
    );
    assert_eq!(units::number(u64::MAX), "18,446,744,073,709,551,615");
    assert_eq!(
        units::number(u128::MAX),
        "340,282,366,920,938,463,463,374,607,431,768,211,455"
    );
    assert_eq!(
        units::number(i128::MIN),
        "-170,141,183,460,469,231,731,687,303,715,884,105,728"
    );
}

#[test]
fn grouped_numbers_preserve_fractions_and_exponents() {
    assert_eq!(units::number(1234.56), "1,234.56");
    assert_eq!(units::number(-10000.0), "-10,000");
    assert_eq!(units::number(1234567.89), "1,234,567.89");
    for (input, expected) in [
        ("+1234567.8900", "+1,234,567.8900"),
        ("12345.670e+8", "12,345.670e+8"),
        ("-12345E-1000", "-12,345E-1000"),
        ("1234.", "1,234."),
        ("0.000012345", "0.000012345"),
    ] {
        assert_eq!(units::number(input), expected);
    }
    assert_eq!(units::number(format_args!("{:e}", 12345.0)), "1.2345e4");
    for n in [0.0, -0.0, f64::MAX, f64::MIN_POSITIVE, f64::from_bits(1)] {
        let output = units::number(n);
        assert_eq!(output.replace(',', ""), n.to_string());
        assert_eq!(units::number(&output), output);
    }
}

#[test]
fn grouped_numbers_leave_other_text_unchanged() {
    for input in [
        "",
        "NaN",
        "inf",
        "-inf",
        "1234e",
        "1234e+",
        "1234x",
        "1234.5.6",
        "1234,567",
        "1234é",
        "１２３４",
        "hello",
    ] {
        assert_eq!(units::number(input), input);
    }
    for input in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(units::number(input), input.to_string());
    }
}

#[test]
fn compact_numbers() {
    for (input, expected) in [
        (0.0, "0"),
        (-0.0, "-0"),
        (999.0, "999"),
        (999.99, "999.99"),
        (1000.0, "1K"),
        (1100.0, "1.1K"),
        (1500.0, "1.5K"),
        (-2500.0, "-2.5K"),
        (999_949.0, "999.9K"),
        (999_950.0, "1M"),
        (999_999.0, "1M"),
        (-999_999.0, "-1M"),
        (1_000_000.0, "1M"),
        (1_010_000.0, "1M"),
        (1_000_000_000.0, "1B"),
        (999_999_999_999.0, "1T"),
        (1_000_000_000_000.0, "1T"),
        (1_000_000_000_000_000.0, "1000T"),
    ] {
        assert_eq!(units::number_short(input), expected, "input: {input}");
    }
}

#[test]
fn compact_numbers_handle_float_extremes() {
    for input in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
    ] {
        assert_eq!(units::number_short(input), input.to_string());
    }
    for input in [1000.0, 999_999.0, 1e20, f64::MAX] {
        let positive = units::number_short(input);
        let negative = units::number_short(-input);
        assert_eq!(negative, format!("-{positive}"));
    }
    assert!(units::number_short(f64::MAX).ends_with('T'));
}

#[test]
fn durations() {
    for (seconds, expected) in [
        (0, "0s"),
        (59, "59s"),
        (60, "1m 0s"),
        (3599, "59m 59s"),
        (3600, "1h 0m 0s"),
        (3661, "1h 1m 1s"),
        (86399, "23h 59m 59s"),
        (86400, "1d 0h 0m 0s"),
        (86401, "1d 0h 0m 1s"),
        (90061, "1d 1h 1m 1s"),
        (u64::MAX, "213503982334601d 7h 0m 15s"),
    ] {
        assert_eq!(units::duration(seconds), expected);
    }
    let value = core::time::Duration::from_millis(1500);
    assert_eq!(units::duration(value.as_secs()), "1s");
}

#[test]
fn module_helpers_are_available_at_the_crate_root() {
    assert_eq!(reword::bytes(1024), "1.0 KiB");
    assert_eq!(reword::bytes_decimal(1000), "1.0 kB");
    assert_eq!(reword::number(1234), "1,234");
    assert_eq!(reword::number_short(1500.0), "1.5K");
    assert_eq!(reword::duration(60), "1m 0s");
}
