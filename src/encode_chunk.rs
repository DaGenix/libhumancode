use crate::error::{error, HumancodeError, HumancodeErrorInfo};
use crate::smallbytebuf::SmallByteBuf;
use libzbase32::low_level_decode::required_octets_buffer_len;
use libzbase32::low_level_encode::{
    octets_to_quintets, quintet_to_character, required_quintets_buffer_len,
};
use reed_solomon_32::Encoder;
use std::fmt::{Debug, Display, Formatter};

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
        std::str::from_utf8(&self.buf.as_bytes())
            .expect("Encoded result couldn't be converted to utf-8 - which shouldn't be possible")
    }
}

impl AsRef<str> for EncodedChunkRaw {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Debug for EncodedChunkRaw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for EncodedChunkRaw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
        std::str::from_utf8(&self.buf.as_bytes())
            .expect("Encoded result couldn't be converted to utf-8 - which shouldn't be possible")
    }
}

impl AsRef<str> for EncodedChunkPretty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Debug for EncodedChunkPretty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for EncodedChunkPretty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    rs_encoder: Encoder,
    ecc: usize,
}

impl ChunkEncoder {
    /// Create a new `ChunkEncoder`.
    ///
    /// `ecc` is the number of error correcting symbols to use. It must
    /// be at least 1 and less than or equal to 30
    pub fn new(ecc: usize) -> Result<ChunkEncoder, HumancodeError> {
        if ecc == 0 || ecc >= 31 {
            return Err(error(HumancodeErrorInfo::InvalidECCLen));
        }
        Ok(ChunkEncoder {
            rs_encoder: Encoder::new(ecc),
            ecc,
        })
    }

    /// Encode a chunk of input data
    ///
    /// `data` must be at least 1 byte long, but no longer than 19 bytes.
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
    pub fn encode_chunk(&self, data: &[u8], bits: u8) -> Result<EncodedChunk, HumancodeError> {
        if data.len() > 19 {
            return Err(error(HumancodeErrorInfo::EncodeBufferTooBig));
        }
        if bits == 0 || bits > 150 {
            return Err(error(HumancodeErrorInfo::InvalidBits));
        }
        if data.len()
            != required_octets_buffer_len(bits as u64)
                .expect("required_octets_buffer_len() failed - which shouldn't be possible")
        {
            return Err(error(HumancodeErrorInfo::EncodeBufferDoesntMatchBits));
        }

        let data_quintets_len = required_quintets_buffer_len(bits as u64)
            .expect("required_quintets_buffer_len() failed - which shouldn't be possible");
        let total_len = data_quintets_len + self.ecc;

        if total_len > 31 {
            return Err(error(HumancodeErrorInfo::TotalEncodeLenTooLong));
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
