use crate::error::{
    encode_buffer_doesnt_match_bits, encode_buffer_had_nonzero_trailing_bits,
    encode_buffer_too_big, invalid_bits, invalid_ecc_len, total_encode_len_too_long, UsageError,
};
use crate::smallbytebuf::SmallByteBuf;
use core::fmt::{Debug, Display, Formatter};
use libzbase32::low_level_decode::required_octets_buffer_len;
use libzbase32::low_level_encode::{
    is_last_octet_valid, octets_to_quintets, quintet_to_character, required_quintets_buffer_len,
};
use reed_solomon_32::encoder as reed_solomoon_encoder;

/// [`ChunkEncoder`] for messages with no error correcting symbols
pub const CHUNK_ENCODER_0: ChunkEncoder = ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_0, 0);
/// [`ChunkEncoder`] for messages with 1 error correcting symbol
pub const CHUNK_ENCODER_1: ChunkEncoder = ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_1, 1);
/// [`ChunkEncoder`] for messages with 2 error correcting symbols
pub const CHUNK_ENCODER_2: ChunkEncoder = ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_2, 2);
/// [`ChunkEncoder`] for messages with 3 error correcting symbols
pub const CHUNK_ENCODER_3: ChunkEncoder = ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_3, 3);
/// [`ChunkEncoder`] for messages with 4 error correcting symbols
pub const CHUNK_ENCODER_4: ChunkEncoder = ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_4, 4);
/// [`ChunkEncoder`] for messages with 5 error correcting symbols
pub const CHUNK_ENCODER_5: ChunkEncoder = ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_5, 5);
/// [`ChunkEncoder`] for messages with 6 error correcting symbols
pub const CHUNK_ENCODER_6: ChunkEncoder = ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_6, 6);
/// [`ChunkEncoder`] for messages with 7 error correcting symbols
pub const CHUNK_ENCODER_7: ChunkEncoder = ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_7, 7);
/// [`ChunkEncoder`] for messages with 8 error correcting symbols
pub const CHUNK_ENCODER_8: ChunkEncoder = ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_8, 8);
/// [`ChunkEncoder`] for messages with 9 error correcting symbols
pub const CHUNK_ENCODER_9: ChunkEncoder = ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_9, 9);
/// [`ChunkEncoder`] for messages with 10 error correcting symbols
pub const CHUNK_ENCODER_10: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_10, 10);
/// [`ChunkEncoder`] for messages with 11 error correcting symbols
pub const CHUNK_ENCODER_11: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_11, 11);
/// [`ChunkEncoder`] for messages with 12 error correcting symbols
pub const CHUNK_ENCODER_12: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_12, 12);
/// [`ChunkEncoder`] for messages with 13 error correcting symbols
pub const CHUNK_ENCODER_13: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_13, 13);
/// [`ChunkEncoder`] for messages with 14 error correcting symbols
pub const CHUNK_ENCODER_14: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_14, 14);
/// [`ChunkEncoder`] for messages with 15 error correcting symbols
pub const CHUNK_ENCODER_15: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_15, 15);
/// [`ChunkEncoder`] for messages with 16 error correcting symbols
pub const CHUNK_ENCODER_16: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_16, 16);
/// [`ChunkEncoder`] for messages with 17 error correcting symbols
pub const CHUNK_ENCODER_17: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_17, 17);
/// [`ChunkEncoder`] for messages with 18 error correcting symbols
pub const CHUNK_ENCODER_18: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_18, 18);
/// [`ChunkEncoder`] for messages with 19 error correcting symbols
pub const CHUNK_ENCODER_19: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_19, 19);
/// [`ChunkEncoder`] for messages with 20 error correcting symbols
pub const CHUNK_ENCODER_20: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_20, 20);
/// [`ChunkEncoder`] for messages with 21 error correcting symbols
pub const CHUNK_ENCODER_21: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_21, 21);
/// [`ChunkEncoder`] for messages with 22 error correcting symbols
pub const CHUNK_ENCODER_22: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_22, 22);
/// [`ChunkEncoder`] for messages with 23 error correcting symbols
pub const CHUNK_ENCODER_23: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_23, 23);
/// [`ChunkEncoder`] for messages with 24 error correcting symbols
pub const CHUNK_ENCODER_24: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_24, 24);
/// [`ChunkEncoder`] for messages with 25 error correcting symbols
pub const CHUNK_ENCODER_25: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_25, 25);
/// [`ChunkEncoder`] for messages with 26 error correcting symbols
pub const CHUNK_ENCODER_26: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_26, 26);
/// [`ChunkEncoder`] for messages with 27 error correcting symbols
pub const CHUNK_ENCODER_27: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_27, 27);
/// [`ChunkEncoder`] for messages with 28 error correcting symbols
pub const CHUNK_ENCODER_28: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_28, 28);
/// [`ChunkEncoder`] for messages with 29 error correcting symbols
pub const CHUNK_ENCODER_29: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_29, 29);
/// [`ChunkEncoder`] for messages with 30 error correcting symbols
pub const CHUNK_ENCODER_30: ChunkEncoder =
    ChunkEncoder::new(&reed_solomoon_encoder::ENCODER_30, 30);

/// An Encoded chunk of bytes
///
/// You can't do much with an `EncodedChunk` directly. Use
/// the [`raw`](EncodedChunk::raw) or [`pretty`](EncodedChunk::pretty) to
/// convert the `EncodedChunk` into a type that can be used.
#[derive(Copy, Clone, Debug)]
pub struct EncodedChunk {
    raw: EncodedChunkRaw,
}

impl EncodedChunk {
    pub(crate) fn from_quintet_buffer(quintet_buffer: &[u8]) -> EncodedChunk {
        let mut encoded_data = [0u8; 31];
        assert!(quintet_buffer.len() <= encoded_data.len());
        for (x, &y) in encoded_data.iter_mut().zip(quintet_buffer.iter()) {
            *x = quintet_to_character(y).expect("quintet_to_character() failed - which shouldn't be possible since we only pass in valid values");
        }
        EncodedChunk {
            raw: EncodedChunkRaw {
                buf: SmallByteBuf::new(encoded_data, quintet_buffer.len() as u8),
            },
        }
    }

    /// Format the `EncodedChunk` with the "raw" format - just the raw z-base-32 characters
    pub fn raw(self) -> EncodedChunkRaw {
        self.raw
    }

    /// Format the `EncodedChunk` with the "pretty" format -
    /// every group of 4 characters will be separated by a "-"
    /// to make the code easier for a human to read.
    pub fn pretty(self) -> EncodedChunkPretty {
        EncodedChunkPretty::from_raw(self.raw)
    }
}

/// A code in the "raw" format - just a list of z-base-32 characters with
/// no separators
#[derive(Copy, Clone)]
pub struct EncodedChunkRaw {
    buf: SmallByteBuf<31>,
}

impl EncodedChunkRaw {
    /// Get the code as a `str`
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf.as_bytes())
            .expect("Encoded result couldn't be converted to utf-8 - which shouldn't be possible")
    }
}

impl AsRef<str> for EncodedChunkRaw {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Debug for EncodedChunkRaw {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for EncodedChunkRaw {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A code in the "pretty" format - every group of 4 characters will be
/// separated by a "-" to make it easier for a human to read.
#[derive(Copy, Clone)]
pub struct EncodedChunkPretty {
    // length 38, because the longest encoded
    // chunk will look something like this:
    // abcd-abcd-abcd-abcd-abcd-abcd-abcd-abc
    buf: SmallByteBuf<38>,
}

impl EncodedChunkPretty {
    // I'm not sure if we want to actually implement From<EncodedChunkRaw>
    // here - so, instead we just us this private method.
    fn from_raw(raw: EncodedChunkRaw) -> EncodedChunkPretty {
        let mut pos = 0;
        let mut encoded_data = [0u8; 38];
        for &x in raw.buf.as_bytes().iter() {
            if (pos + 1) % 5 == 0 {
                encoded_data[pos] = b'-';
                pos += 1;
            }
            encoded_data[pos] = x;
            pos += 1;
        }
        EncodedChunkPretty {
            buf: SmallByteBuf::new(encoded_data, pos as u8),
        }
    }

    /// Get the code as a `str`
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf.as_bytes())
            .expect("Encoded result couldn't be converted to utf-8 - which shouldn't be possible")
    }
}

impl AsRef<str> for EncodedChunkPretty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Debug for EncodedChunkPretty {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for EncodedChunkPretty {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A `ChunkEncoder` is able to encode up to 150 bits of data with a configurable
/// number of error correcting symbols.
// We don't implement Copy / Clone because Encoder doesn't. It
// probably should - so if its updated we could as well. Its
// unclear how valuable that really is, however.
#[derive(Debug)]
pub struct ChunkEncoder {
    rs_encoder: &'static reed_solomoon_encoder::Encoder,
    ecc: u8,
}

impl ChunkEncoder {
    /// Create a new `ChunkEncoder`.
    ///
    /// `ecc` is the number of error correcting symbols to use. It must
    /// be at least 1 and less than or equal to 30
    const fn new(rs_encoder: &'static reed_solomoon_encoder::Encoder, ecc: u8) -> ChunkEncoder {
        ChunkEncoder { rs_encoder, ecc }
    }

    /// Encode a chunk of input data
    ///
    /// `data` must be at least 1 byte long, but no longer than 20 bytes.
    ///
    /// `bits` indicates the number of bits to encode from `data`. `data` must
    /// have a minimal length given the number of `bits`. For example, if `bits`
    /// is 5, `data` must be 1 byte long. If `bits` is 9, `data` must be 2 bytes long.
    ///
    /// `bits` must be at least 1 and less than or equal to 150.
    ///
    /// `data` is encoded in a big-endian fashion. So, if `bits` is 1 - only the
    /// _highest_ bit of `data` will be encoded. All remaining bits of `data` must
    /// be 0s or an error will be reported.
    pub fn encode_chunk(&self, data: &[u8], bits: u8) -> Result<EncodedChunk, UsageError> {
        if data.len() > 20 {
            return Err(encode_buffer_too_big());
        }
        if bits == 0 || bits > 155 {
            return Err(invalid_bits());
        }
        if data.len()
            != required_octets_buffer_len(bits as u64)
                .expect("required_octets_buffer_len() failed - which shouldn't be possible")
        {
            return Err(encode_buffer_doesnt_match_bits());
        }

        let data_quintets_len = required_quintets_buffer_len(bits as u64)
            .expect("required_quintets_buffer_len() failed - which shouldn't be possible");
        let total_len = data_quintets_len + self.ecc as usize;

        if total_len > 31 {
            return Err(total_encode_len_too_long());
        }

        if !is_last_octet_valid(bits as u64, data[data.len() - 1]) {
            return Err(encode_buffer_had_nonzero_trailing_bits());
        }

        let mut quintets_buffer = SmallByteBuf::new([0u8; 31], data_quintets_len as u8);
        octets_to_quintets(data, quintets_buffer.as_mut_bytes(), bits as u64)
            .expect("octets_to_quintets() failed - which shouldn't be possible");

        let rs_encoded_buffer = self
            .rs_encoder
            .encode(&quintets_buffer.as_bytes())
            .expect("Reed Solomon 32 encode failed - which shouldn't be possible");

        Ok(EncodedChunk::from_quintet_buffer(&rs_encoded_buffer))
    }
}

/// Encode a chunk of input data
///
/// `data` must be at least 1 byte long, but no longer than 20 bytes.
///
/// `ecc` indicates the number of error correcting symbols to use and must
/// between 0 and 30, inclusive.
///
/// `bits` indicates the number of bits to encode from `data`. `data` must
/// have a minimal length given the number of `bits`. For example, if `bits`
/// is 5, `data` must be 1 byte long. If `bits` is 9, `data` must be 2 bytes long.
///
/// `bits` must be at least 1 and less than or equal to 150.
///
/// `data` is encoded in a big-endian fashion. So, if `bits` is 1 - only the
/// _highest_ bit of `data` will be encoded. All remaining bits of `data` must
/// be 0s or an error will be reported.
pub fn encode_chunk(data: &[u8], ecc: u8, bits: u8) -> Result<EncodedChunk, UsageError> {
    match ecc {
        0 => CHUNK_ENCODER_0.encode_chunk(data, bits),
        1 => CHUNK_ENCODER_1.encode_chunk(data, bits),
        2 => CHUNK_ENCODER_2.encode_chunk(data, bits),
        3 => CHUNK_ENCODER_3.encode_chunk(data, bits),
        4 => CHUNK_ENCODER_4.encode_chunk(data, bits),
        5 => CHUNK_ENCODER_5.encode_chunk(data, bits),
        6 => CHUNK_ENCODER_6.encode_chunk(data, bits),
        7 => CHUNK_ENCODER_7.encode_chunk(data, bits),
        8 => CHUNK_ENCODER_8.encode_chunk(data, bits),
        9 => CHUNK_ENCODER_9.encode_chunk(data, bits),
        10 => CHUNK_ENCODER_10.encode_chunk(data, bits),
        11 => CHUNK_ENCODER_11.encode_chunk(data, bits),
        12 => CHUNK_ENCODER_12.encode_chunk(data, bits),
        13 => CHUNK_ENCODER_13.encode_chunk(data, bits),
        14 => CHUNK_ENCODER_14.encode_chunk(data, bits),
        15 => CHUNK_ENCODER_15.encode_chunk(data, bits),
        16 => CHUNK_ENCODER_16.encode_chunk(data, bits),
        17 => CHUNK_ENCODER_17.encode_chunk(data, bits),
        18 => CHUNK_ENCODER_18.encode_chunk(data, bits),
        19 => CHUNK_ENCODER_19.encode_chunk(data, bits),
        20 => CHUNK_ENCODER_20.encode_chunk(data, bits),
        21 => CHUNK_ENCODER_21.encode_chunk(data, bits),
        22 => CHUNK_ENCODER_22.encode_chunk(data, bits),
        23 => CHUNK_ENCODER_23.encode_chunk(data, bits),
        24 => CHUNK_ENCODER_24.encode_chunk(data, bits),
        25 => CHUNK_ENCODER_25.encode_chunk(data, bits),
        26 => CHUNK_ENCODER_26.encode_chunk(data, bits),
        27 => CHUNK_ENCODER_27.encode_chunk(data, bits),
        28 => CHUNK_ENCODER_28.encode_chunk(data, bits),
        29 => CHUNK_ENCODER_29.encode_chunk(data, bits),
        30 => CHUNK_ENCODER_30.encode_chunk(data, bits),
        _ => Err(invalid_ecc_len()),
    }
}
