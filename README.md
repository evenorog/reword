# reword

[![Rust](https://github.com/evenorog/reword/actions/workflows/rust.yml/badge.svg)](https://github.com/evenorog/reword/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/reword.svg)](https://crates.io/crates/reword)
[![Docs](https://docs.rs/reword/badge.svg)](https://docs.rs/reword)

Provides utility functions for human readable formatting of words and sentences.

```rust
const S: &str = "(Even),Olsson&Rogstadkjærnet?";

assert_eq!(reword::name(S), "Even Olsson Rogstadkjærnet");
assert_eq!(reword::name_with_limit(S, 4), "EOR");
assert_eq!(reword::username_with_limit(S, 12), "evenor");
assert_eq!(reword::camel_case(S), "evenOlssonRogstadkjærnet");
assert_eq!(reword::upper_camel_case_with_limit(S, 25), "EvenORogstadkjærnet");
```

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
