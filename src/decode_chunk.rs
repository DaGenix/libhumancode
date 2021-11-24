use crate::error::{error, HumancodeError, HumancodeErrorInfo};
use crate::smallbytebuf::SmallByteBuf;
use crate::EncodedChunk;
use core::fmt::{Debug, Formatter};
use libzbase32::low_level_decode::{
    character_to_quintet, is_last_quintet_valid, quintets_to_octets, required_octets_buffer_len,
};
use libzbase32::low_level_encode::required_quintets_buffer_len;
use libzbase32::ZBase32ErrorType;
use reed_solomon_32::Decoder;

/// A decoded chunk of bytes
///
/// The [`as_bytes`](DecodedChunk::as_bytes) method can be used to
/// access the underlying bytes.
#[derive(Copy, Clone)]
pub struct DecodedChunk {
    buf: SmallByteBuf<19>,
}

impl DecodedChunk {
    /// Get the underlying decoded bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.buf.as_bytes()
    }
}

impl AsRef<[u8]> for DecodedChunk {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Debug for DecodedChunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.as_bytes())
    }
}

/// A `ChunkDecoder` can decode a string encoded with [`ChunkEncoder`](crate::ChunkEncoder)
/// and report on any errors that were found / corrected.
// We don't implement Copy / Clone because ChunkEncoder currently can't
// and we want to be consistent.
#[derive(Debug)]
pub struct ChunkDecoder {
    rs_decoder: Decoder,
    ecc: usize,
}

impl ChunkDecoder {
    /// Create a new `ChunkDecoder`.
    ///
    /// `ecc` must match the corresponding [`ChunkEncoder`](crate::ChunkEncoder)
    /// and must be greater than 0 and less or equal to 30.
    pub fn new(ecc: usize) -> Result<ChunkDecoder, HumancodeError> {
        if ecc == 0 || ecc >= 31 {
            return Err(error(HumancodeErrorInfo::InvalidECCLen));
        }
        Ok(ChunkDecoder {
            rs_decoder: Decoder::new(ecc),
            ecc,
        })
    }

    /// Decode and correct a chunk encoded by [`encode_chunk`](crate::ChunkEncoder::encode_chunk)
    ///
    /// `bits` much match the value that was passed to the `ChunkEncoder`.
    ///
    /// `encoded_data` should be a value returned by `ChunkEncoder`. `encoded_data`
    /// may include any number of "-" characters which will be ignored.
    ///
    /// `encoded_data` should be validated for the correct length prior to being
    /// passed to this method. Incorrect lengths will result in errors of
    /// type [`UsageError`](crate::HumancodeErrorType::UsageError).
    ///
    /// On success, a tuple of [`DecodedChunk`] and an Optional [`EncodedChunk`]
    /// is returned. The `EncodedChunk` will only be a `Some` value if there was
    /// an error in the input that was corrected. It is strongly recommended that
    /// the user be prompted to review any errors.
    pub fn decode_chunk(
        &self,
        encoded_data: &str,
        bits: u8,
    ) -> Result<(DecodedChunk, Option<EncodedChunk>), HumancodeError> {
        if bits == 0 || bits > 150 {
            return Err(error(HumancodeErrorInfo::InvalidBits));
        }

        fn convert_encoded_data_to_quintets(
            bits: u8,
            num_quintets: usize,
            encoded_data: &str,
        ) -> Result<(SmallByteBuf<31>, SmallByteBuf<31>), HumancodeError> {
            let mut out_buffer = [0u8; 31];
            let mut out_idx = 0;
            let mut erase_pos = [0u8; 31];
            let mut erase_pos_size = 0;

            for &x in encoded_data.as_bytes().iter() {
                if x == b'-' {
                    continue;
                }

                if out_idx >= out_buffer.len() {
                    return Err(error(HumancodeErrorInfo::DecoderBufferTooBig));
                }

                match character_to_quintet(x) {
                    Ok(x) => {
                        if out_idx + 1 == num_quintets && !is_last_quintet_valid(bits as u64, x) {
                            // If we're dealing with the last quintet of the data payload,
                            // we have to check if its valid given the bits size - since
                            // libzbase32 doesn't permit for trailing non-zero bits
                            erase_pos[erase_pos_size] = out_idx as u8;
                            erase_pos_size += 1;
                        } else {
                            out_buffer[out_idx] = x;
                        }
                    }
                    Err(err) => match err.error_type() {
                        ZBase32ErrorType::InputError => {
                            // If the input character is invalid, we can record
                            // it as an erasure which helps when we apply error
                            // correction later.
                            erase_pos[erase_pos_size] = out_idx as u8;
                            erase_pos_size += 1;
                        }
                        ZBase32ErrorType::UsageError => {
                            unreachable!("This shouldn't be possible")
                        }
                    },
                };
                out_idx += 1;
            }

            Ok((
                SmallByteBuf::new(out_buffer, out_idx as u8),
                SmallByteBuf::new(erase_pos, erase_pos_size as u8),
            ))
        }

        let num_quintets = required_quintets_buffer_len(bits as u64)
            .expect("required_quintets_buffer_len() failed - which shouldn't be possible");

        let (quintet_buffer, erase_pos) =
            convert_encoded_data_to_quintets(bits, num_quintets, encoded_data)?;
        if quintet_buffer.len() <= self.ecc {
            return Err(error(HumancodeErrorInfo::DecodeBufferSmallerThanEcc));
        }

        if quintet_buffer.len() - self.ecc != num_quintets {
            return Err(error(HumancodeErrorInfo::DecodeBufferWrongSize));
        }

        let (out, err_count) = match self
            .rs_decoder
            .correct_err_count(quintet_buffer.as_bytes(), Some(erase_pos.as_bytes()))
        {
            Ok(r) => r,
            Err(_) => return Err(error(HumancodeErrorInfo::TooManyErrors)),
        };

        let corrected_chunk = if err_count > 0 || erase_pos.len() > 0 {
            Some(EncodedChunk::from_quintet_buffer(&out))
        } else {
            None
        };

        let decoded_data_len = required_octets_buffer_len(bits as u64)
            .expect("required_octets_buffer_len() failed - which shouldn't be possible");
        let mut decoded_chunk = DecodedChunk {
            buf: SmallByteBuf::new([0u8; 19], decoded_data_len as u8),
        };

        if err_count > 0 || erase_pos.len() > 0 {
            // If we have some errors, then its possible that our corrected code
            // is actually wrong. This could cause the final quintet to be an
            // invalid value for the number of bits. If so, we need to check
            // for that condition here - otherwise quintets_to_octets() will
            // fail below.
            let final_data_quintet = out.data()[out.data().len() - 1];
            if !is_last_quintet_valid(bits as u64, final_data_quintet) {
                return Err(error(HumancodeErrorInfo::TooManyErrors));
            }
        }

        // This function only fails if the quintets are invalid (ie, >31) or if the final
        // quintet is not valid for the given bits value. We've already ensured that
        // neither of those things can be true, so, this shouldn't be able to fail.
        quintets_to_octets(out.data(), decoded_chunk.buf.as_mut_bytes(), bits as u64)
            .expect("quintets_to_octets() failed - which shouldn't be possible");

        Ok((decoded_chunk, corrected_chunk))
    }
}
