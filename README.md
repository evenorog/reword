# reword

[![Travis](https://travis-ci.com/evenorog/reword.svg?branch=master)](https://travis-ci.com/evenorog/reword)
[![Crates.io](https://img.shields.io/crates/v/reword.svg)](https://crates.io/crates/reword)
[![Docs](https://docs.rs/reword/badge.svg)](https://docs.rs/reword)

Provides functions for human readable formatting of words and sentences.

```rust
let s = "(Even),Olsson&Rogstadkjærnet?";
assert_eq!(reword::name(s), "Even Olsson Rogstadkjærnet");
assert_eq!(reword::name_with_limit(s, 4), "EOR");
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
