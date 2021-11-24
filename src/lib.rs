//! # libhumancode
//!
//! libhumancode is a `no_std` compatible crate that provides a
//! mechanism to encode up to 150 bits of binary data in a human
//! friendly format.
//!
//! z-base-32 encoding is used to encode all data - this allows for
//! using a minimal number of symbols to encode data (unlike regular
//! base-32, which requires padding characters depending on the number
//! of bits to encode). Additionally, z-base-32 is designed to be human
//! friendly. The tradeoff is that the sender and receiver of a code must
//! agree on the number of bits of data in each code.
//!
//! libhumancode also uses a configurable number of error correction
//! symbols using a Reed Solomon GF(2^5) code. For each error correcting
//! symbol added, this means that we can detect at least 1 error in
//! a code. For every two symbols added, we can correct an error. Note
//! that these properties are not additive - with 5 error correcting
//! symbols, if we have an input with 2 errors, we will always correct
//! it. If we have an input with 3 errors, we will always report it as
//! incorrect. However, if we have an input with 4 errors, we might
//! accidentally "correct" it to an invalid code. As such, its highly
//! recommended to confirm code corrections with the user.
//!
//! ## Example
//!
//! ```
//! use libhumancode::{decode_chunk, encode_chunk};
//!
//! fn main() {
//!     const DATA: &'static [u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
//!     const CORRECT_CODE: &'static str = "yyyo-ryar-ywdy-qnyj-befo-adeq-bhix-4os";
//!     const INVALID_CODE: &'static str = "!!yo-ryar-ywdy-qnyj-befo-adeq-bhix-4os";
//!
//!     let encoded = encode_chunk(DATA, 5, 128).unwrap();
//!     let encoded_pretty = encoded.pretty();
//!
//!     assert_eq!(encoded_pretty.as_str(), CORRECT_CODE);
//!
//!     let (decoded, corrected) = decode_chunk(INVALID_CODE, 5, 128).unwrap();
//!
//!     assert_eq!(decoded.as_bytes(), DATA);
//!     assert_eq!(corrected.unwrap().pretty().as_str(), CORRECT_CODE);
//! }
//! ```
//!
//! ## No_std
//!
//! No_std mode may be activated by disabling the "std" feature.
//!
//! ## License
//!
//! This project is licensed under either of
//!
//! * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
//!   <https://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or
//!   <https://opensource.org/licenses/MIT>)
//!
//! at your option.

#![cfg_attr(not(feature = "std"), no_std)]

mod decode_chunk;
mod encode_chunk;
mod error;
mod smallbytebuf;

#[cfg(test)]
mod tests;

pub use decode_chunk::{decode_chunk, DecodedChunk};
pub use encode_chunk::{encode_chunk, EncodedChunk, EncodedChunkPretty, EncodedChunkRaw};
pub use error::{HumancodeError, InputErrorCause, UsageErrorCause};

pub mod decoder {
    //! Using the [`ChunkDecoder`] interfaces in this module _may_ allow for a smaller binary size
    //! in the future. However, currently there is no size advantage to using these
    //! interfaces over [`crate::decode_chunk()`]. These interfaces primarily exist
    //! for consistency with the API of the [`crate::encoder`] module.
    pub use crate::decode_chunk::{
        ChunkDecoder, CHUNK_DECODER_0, CHUNK_DECODER_1, CHUNK_DECODER_10, CHUNK_DECODER_11,
        CHUNK_DECODER_12, CHUNK_DECODER_13, CHUNK_DECODER_14, CHUNK_DECODER_15, CHUNK_DECODER_16,
        CHUNK_DECODER_17, CHUNK_DECODER_18, CHUNK_DECODER_19, CHUNK_DECODER_2, CHUNK_DECODER_20,
        CHUNK_DECODER_21, CHUNK_DECODER_22, CHUNK_DECODER_23, CHUNK_DECODER_24, CHUNK_DECODER_25,
        CHUNK_DECODER_26, CHUNK_DECODER_27, CHUNK_DECODER_28, CHUNK_DECODER_29, CHUNK_DECODER_3,
        CHUNK_DECODER_30, CHUNK_DECODER_4, CHUNK_DECODER_5, CHUNK_DECODER_6, CHUNK_DECODER_7,
        CHUNK_DECODER_8, CHUNK_DECODER_9,
    };
}

pub mod encoder {
    //! Using the [`ChunkEncoder`] interfaces _may_ allow for a smaller binary size
    //! since it _may_ allow for certain pre-calculated tables to be removed at
    //! build time. This won't work for all targets and at best can save about 1k
    //! over using [`crate::encode_chunk()`] directly.
    pub use crate::encode_chunk::{
        ChunkEncoder, CHUNK_ENCODER_0, CHUNK_ENCODER_1, CHUNK_ENCODER_10, CHUNK_ENCODER_11,
        CHUNK_ENCODER_12, CHUNK_ENCODER_13, CHUNK_ENCODER_14, CHUNK_ENCODER_15, CHUNK_ENCODER_16,
        CHUNK_ENCODER_17, CHUNK_ENCODER_18, CHUNK_ENCODER_19, CHUNK_ENCODER_2, CHUNK_ENCODER_20,
        CHUNK_ENCODER_21, CHUNK_ENCODER_22, CHUNK_ENCODER_23, CHUNK_ENCODER_24, CHUNK_ENCODER_25,
        CHUNK_ENCODER_26, CHUNK_ENCODER_27, CHUNK_ENCODER_28, CHUNK_ENCODER_29, CHUNK_ENCODER_3,
        CHUNK_ENCODER_30, CHUNK_ENCODER_4, CHUNK_ENCODER_5, CHUNK_ENCODER_6, CHUNK_ENCODER_7,
        CHUNK_ENCODER_8, CHUNK_ENCODER_9,
    };
}
