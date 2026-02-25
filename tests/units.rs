
#[test]
fn bytes() {
    assert_eq!(reword::bytes(0), "0 B");
    assert_eq!(reword::bytes(1023), "1023 B");
    assert_eq!(reword::bytes(1024), "1.0 KB");
    assert_eq!(reword::bytes(1024 * 1024), "1.0 MB");
    assert_eq!(reword::bytes(1024 * 1024 * 1024), "1.0 GB");
}

#[test]
fn number() {
    assert_eq!(reword::number(0.0), "0");
    assert_eq!(reword::number(1234.0), "1,234");
    assert_eq!(reword::number(1234567.89), "1,234,567.89");
    assert_eq!(reword::number(-1234.5), "-1,234.5");
}

#[test]
fn number_short() {
    assert_eq!(reword::number_short(0.0), "0");
    assert_eq!(reword::number_short(999.0), "999");
    assert_eq!(reword::number_short(1000.0), "1K");
    assert_eq!(reword::number_short(1100.0), "1.1K");
    assert_eq!(reword::number_short(1000000.0), "1M");
    assert_eq!(reword::number_short(1000000000.0), "1B");
}

#[test]
fn duration() {
    assert_eq!(reword::duration(0), "0s");
    assert_eq!(reword::duration(59), "59s");
    assert_eq!(reword::duration(60), "1m 0s");
    assert_eq!(reword::duration(3600), "1h 0m 0s");
    assert_eq!(reword::duration(86400), "1d 0h 0m 0s");
    assert_eq!(reword::duration(86400 + 3600 + 60 + 1), "1d 1h 1m 1s");
}
