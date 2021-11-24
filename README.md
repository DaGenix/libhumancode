# libhumancode

[![Crates.io](https://img.shields.io/crates/v/libhumancode.svg)](https://crates.io/crates/libhumancode)
[![Documentation](https://docs.rs/libhumancode/badge.svg)](https://docs.rs/libhumancode)

libhumancode is a `no_std` compatible crate that provides a
mechanism to encode up to 150 bits of binary data in a human
friendly format.

z-base-32 encoding is used to encode all data - this allows for
using a minimal number of symbols to encode data (unlike regular
base-32, which requires padding characters depending on the number
of bits to encode). Additionally, z-base-32 is designed to be human
friendly. The tradeoff is that the sender and receiver of a code must
agree on the number of bits of data in each code.

libhumancode also uses a configurable number of error correction
symbols using a Reed Solomon GF(2^5) code. For each error correcting
symbol added, this means that we can detect at least 1 error in
a code. For every two symbols added, we can correct an error. Note
that these properties are not additive - with 5 error correcting
symbols, if we have an input with 2 errors, we will always correct
it. If we have an input with 3 errors, we will always report it as
incorrect. However, if we have an input with 4 errors, we might
accidentally "correct" it to an invalid code. As such, its highly
recommended to confirm code corrections with the user.

## Example

```rust
use libhumancode::{decode_chunk, encode_chunk};

fn main() {
    const DATA: &'static [u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    const ECC_SYMBOLS: u8 = 5;
    const BITS: u8 = 128;
    const CORRECT_CODE: &'static str = "yyyo-ryar-ywdy-qnyj-befo-adeq-bhix-4os";
    const INVALID_CODE: &'static str = "!!yo-ryar-ywdy-qnyj-befo-adeq-bhix-4os";

    let encoded = encode_chunk(DATA, ECC_SYMBOLS, BITS).unwrap();
    let encoded_pretty = encoded.pretty();

    assert_eq!(encoded_pretty.as_str(), CORRECT_CODE);

    let (decoded, corrected) = decode_chunk(INVALID_CODE, ECC_SYMBOLS, BITS).unwrap();

    assert_eq!(decoded.as_bytes(), DATA);
    assert_eq!(corrected.unwrap().pretty().as_str(), CORRECT_CODE);
}
```

## No_std

No_std mode may be activated by disabling the "std" feature.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

at your option.
