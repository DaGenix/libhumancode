use crate::error::{
    decode_buffer_smaller_than_ecc, decode_buffer_too_big, decode_buffer_wrong_size, invalid_bits,
    invalid_ecc_len, too_many_errors, DecodeError, UsageError,
};
use crate::smallbytebuf::SmallByteBuf;
use crate::EncodedChunk;
use core::fmt::Debug;
use libzbase32::low_level_decode::{
    character_to_quintet, is_last_quintet_valid, quintets_to_octets, required_octets_buffer_len,
};
use libzbase32::low_level_encode::required_quintets_buffer_len;
use libzbase32::InputError;
use reed_solomon_32::decoder as reed_solomoon_decoder;

/// [`ChunkDecoder`] for messages with no error correcting symbols
pub const CHUNK_DECODER_0: ChunkDecoder = ChunkDecoder::new(&reed_solomoon_decoder::DECODER_0, 0);
/// [`ChunkDecoder`] for messages with 1 error correcting symbol
pub const CHUNK_DECODER_1: ChunkDecoder = ChunkDecoder::new(&reed_solomoon_decoder::DECODER_1, 1);
/// [`ChunkDecoder`] for messages with 2 error correcting symbols
pub const CHUNK_DECODER_2: ChunkDecoder = ChunkDecoder::new(&reed_solomoon_decoder::DECODER_2, 2);
/// [`ChunkDecoder`] for messages with 3 error correcting symbols
pub const CHUNK_DECODER_3: ChunkDecoder = ChunkDecoder::new(&reed_solomoon_decoder::DECODER_3, 3);
/// [`ChunkDecoder`] for messages with 4 error correcting symbols
pub const CHUNK_DECODER_4: ChunkDecoder = ChunkDecoder::new(&reed_solomoon_decoder::DECODER_4, 4);
/// [`ChunkDecoder`] for messages with 5 error correcting symbols
pub const CHUNK_DECODER_5: ChunkDecoder = ChunkDecoder::new(&reed_solomoon_decoder::DECODER_5, 5);
/// [`ChunkDecoder`] for messages with 6 error correcting symbols
pub const CHUNK_DECODER_6: ChunkDecoder = ChunkDecoder::new(&reed_solomoon_decoder::DECODER_6, 6);
/// [`ChunkDecoder`] for messages with 7 error correcting symbols
pub const CHUNK_DECODER_7: ChunkDecoder = ChunkDecoder::new(&reed_solomoon_decoder::DECODER_7, 7);
/// [`ChunkDecoder`] for messages with 8 error correcting symbols
pub const CHUNK_DECODER_8: ChunkDecoder = ChunkDecoder::new(&reed_solomoon_decoder::DECODER_8, 8);
/// [`ChunkDecoder`] for messages with 9 error correcting symbols
pub const CHUNK_DECODER_9: ChunkDecoder = ChunkDecoder::new(&reed_solomoon_decoder::DECODER_9, 9);
/// [`ChunkDecoder`] for messages with 10 error correcting symbols
pub const CHUNK_DECODER_10: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_10, 10);
/// [`ChunkDecoder`] for messages with 11 error correcting symbols
pub const CHUNK_DECODER_11: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_11, 11);
/// [`ChunkDecoder`] for messages with 12 error correcting symbols
pub const CHUNK_DECODER_12: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_12, 12);
/// [`ChunkDecoder`] for messages with 13 error correcting symbols
pub const CHUNK_DECODER_13: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_13, 13);
/// [`ChunkDecoder`] for messages with 14 error correcting symbols
pub const CHUNK_DECODER_14: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_14, 14);
/// [`ChunkDecoder`] for messages with 15 error correcting symbols
pub const CHUNK_DECODER_15: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_15, 15);
/// [`ChunkDecoder`] for messages with 16 error correcting symbols
pub const CHUNK_DECODER_16: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_16, 16);
/// [`ChunkDecoder`] for messages with 17 error correcting symbols
pub const CHUNK_DECODER_17: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_17, 17);
/// [`ChunkDecoder`] for messages with 18 error correcting symbols
pub const CHUNK_DECODER_18: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_18, 18);
/// [`ChunkDecoder`] for messages with 19 error correcting symbols
pub const CHUNK_DECODER_19: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_19, 19);
/// [`ChunkDecoder`] for messages with 20 error correcting symbols
pub const CHUNK_DECODER_20: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_20, 20);
/// [`ChunkDecoder`] for messages with 21 error correcting symbols
pub const CHUNK_DECODER_21: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_21, 21);
/// [`ChunkDecoder`] for messages with 22 error correcting symbols
pub const CHUNK_DECODER_22: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_22, 22);
/// [`ChunkDecoder`] for messages with 23 error correcting symbols
pub const CHUNK_DECODER_23: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_23, 23);
/// [`ChunkDecoder`] for messages with 24 error correcting symbols
pub const CHUNK_DECODER_24: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_24, 24);
/// [`ChunkDecoder`] for messages with 25 error correcting symbols
pub const CHUNK_DECODER_25: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_25, 25);
/// [`ChunkDecoder`] for messages with 26 error correcting symbols
pub const CHUNK_DECODER_26: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_26, 26);
/// [`ChunkDecoder`] for messages with 27 error correcting symbols
pub const CHUNK_DECODER_27: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_27, 27);
/// [`ChunkDecoder`] for messages with 28 error correcting symbols
pub const CHUNK_DECODER_28: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_28, 28);
/// [`ChunkDecoder`] for messages with 29 error correcting symbols
pub const CHUNK_DECODER_29: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_29, 29);
/// [`ChunkDecoder`] for messages with 30 error correcting symbols
pub const CHUNK_DECODER_30: ChunkDecoder =
    ChunkDecoder::new(&reed_solomoon_decoder::DECODER_30, 30);

/// The result of a successful decode_chunk operation
#[derive(Debug, Clone, Copy)]
pub struct DecodeOutput {
    data_buf: SmallByteBuf<20>,
    had_errors: bool,
    quintet_buffer: SmallByteBuf<31>,
}

impl DecodeOutput {
    /// Get the decoded (and corrected) data value
    pub fn data(&self) -> &[u8] {
        self.data_buf.as_bytes()
    }

    /// Returns `true` if there were errors in the code that were
    /// corrected
    pub fn had_errors(&self) -> bool {
        self.had_errors
    }

    /// Get the value of the encoded string _after_ corrections
    /// have been applied.
    ///
    /// This is useful in order to ask the user if the corrections
    /// were accurate.
    pub fn corrected_chunk(&self) -> EncodedChunk {
        EncodedChunk::from_quintet_buffer(self.quintet_buffer.as_bytes())
    }
}

/// A `ChunkDecoder` can decode an encoded string
/// and report on any errors that were found / corrected.
// We don't implement Copy / Clone because ChunkEncoder currently can't
// and we want to be consistent.
#[derive(Debug)]
pub struct ChunkDecoder {
    rs_decoder: &'static reed_solomoon_decoder::Decoder,
    ecc: u8,
}

impl ChunkDecoder {
    const fn new(rs_decoder: &'static reed_solomoon_decoder::Decoder, ecc: u8) -> ChunkDecoder {
        ChunkDecoder { rs_decoder, ecc }
    }

    /// Decode and correct an encoded message.
    ///
    /// `bits` much match the value that was passed to [`encode_chunk`](crate::encode_chunk())
    ///
    /// `encoded_data` should be a value returned by `encode_chunk`. `encoded_data`
    /// may include any number of "-" characters which will be ignored.
    ///
    /// `encoded_data` should be validated for the correct length prior to being
    /// passed to this method. Incorrect lengths will result in errors of
    /// type [`UsageError`](crate::DecodeError::UsageError).
    ///
    /// On success, a [`DecodeOutput`] value is returned.
    pub fn decode_chunk(&self, encoded_data: &str, bits: u8) -> Result<DecodeOutput, DecodeError> {
        if bits == 0 || bits > 155 {
            return Err(invalid_bits().into());
        }

        fn convert_encoded_data_to_quintets(
            bits: u8,
            num_quintets: usize,
            encoded_data: &str,
        ) -> Result<(SmallByteBuf<31>, SmallByteBuf<31>), UsageError> {
            let mut out_buffer = [0u8; 31];
            let mut out_idx = 0;
            let mut erase_pos = [0u8; 31];
            let mut erase_pos_size = 0;

            for &x in encoded_data.as_bytes().iter() {
                if x == b'-' {
                    continue;
                }

                if out_idx >= out_buffer.len() {
                    return Err(decode_buffer_too_big());
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
                    Err(InputError::InputError(_)) => {
                        // If the input character is invalid, we can record
                        // it as an erasure which helps when we apply error
                        // correction later.
                        erase_pos[erase_pos_size] = out_idx as u8;
                        erase_pos_size += 1;
                    }
                    Err(InputError::UsageError(_)) => {
                        unreachable!("This shouldn't be possible")
                    }
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
        if quintet_buffer.len() <= self.ecc as usize {
            return Err(decode_buffer_smaller_than_ecc().into());
        }

        if quintet_buffer.len() - self.ecc as usize != num_quintets {
            return Err(decode_buffer_wrong_size().into());
        }

        let (out, err_count) = match self
            .rs_decoder
            .correct_err_count(quintet_buffer.as_bytes(), Some(erase_pos.as_bytes()))
        {
            Ok(r) => r,
            Err(_) => return Err(too_many_errors()),
        };

        let quintet_buffer = SmallByteBuf::from([0u8; 31], &out);

        let decoded_data_len = required_octets_buffer_len(bits as u64)
            .expect("required_octets_buffer_len() failed - which shouldn't be possible");
        let mut data_buf = SmallByteBuf::new([0u8; 20], decoded_data_len as u8);

        if err_count > 0 || erase_pos.len() > 0 {
            // If we have some errors, then its possible that our corrected code
            // is actually wrong. This could cause the final quintet to be an
            // invalid value for the number of bits. If so, we need to check
            // for that condition here - otherwise quintets_to_octets() will
            // fail below.
            let final_data_quintet = out.data()[out.data().len() - 1];
            if !is_last_quintet_valid(bits as u64, final_data_quintet) {
                return Err(too_many_errors());
            }
        }

        // This function only fails if the quintets are invalid (ie, >31) or if the final
        // quintet is not valid for the given bits value. We've already ensured that
        // neither of those things can be true, so, this shouldn't be able to fail.
        quintets_to_octets(out.data(), data_buf.as_mut_bytes(), bits as u64)
            .expect("quintets_to_octets() failed - which shouldn't be possible");

        Ok(DecodeOutput {
            data_buf,
            had_errors: err_count > 0 || erase_pos.len() > 0,
            quintet_buffer,
        })
    }
}

/// Decode and correct an encoded message.
///
/// `bits` much match the value that was passed to [`encode_chunk`](crate::encode_chunk())
///
/// `encoded_data` should be a value returned by `encode_chunk`. `encoded_data`
/// may include any number of "-" characters which will be ignored.
///
/// `encoded_data` should be validated for the correct length prior to being
/// passed to this method. Incorrect lengths will result in errors of
/// type [`UsageError`](crate::DecodeError::UsageError).
///
/// `ecc` indicates the number of error correcting symbols to use and must
/// match the value passed to `encode_chunk`.
///
/// On success, a [`DecodeOutput`] value is returned.
pub fn decode_chunk(encoded_data: &str, ecc: u8, bits: u8) -> Result<DecodeOutput, DecodeError> {
    match ecc {
        0 => CHUNK_DECODER_0.decode_chunk(encoded_data, bits),
        1 => CHUNK_DECODER_1.decode_chunk(encoded_data, bits),
        2 => CHUNK_DECODER_2.decode_chunk(encoded_data, bits),
        3 => CHUNK_DECODER_3.decode_chunk(encoded_data, bits),
        4 => CHUNK_DECODER_4.decode_chunk(encoded_data, bits),
        5 => CHUNK_DECODER_5.decode_chunk(encoded_data, bits),
        6 => CHUNK_DECODER_6.decode_chunk(encoded_data, bits),
        7 => CHUNK_DECODER_7.decode_chunk(encoded_data, bits),
        8 => CHUNK_DECODER_8.decode_chunk(encoded_data, bits),
        9 => CHUNK_DECODER_9.decode_chunk(encoded_data, bits),
        10 => CHUNK_DECODER_10.decode_chunk(encoded_data, bits),
        11 => CHUNK_DECODER_11.decode_chunk(encoded_data, bits),
        12 => CHUNK_DECODER_12.decode_chunk(encoded_data, bits),
        13 => CHUNK_DECODER_13.decode_chunk(encoded_data, bits),
        14 => CHUNK_DECODER_14.decode_chunk(encoded_data, bits),
        15 => CHUNK_DECODER_15.decode_chunk(encoded_data, bits),
        16 => CHUNK_DECODER_16.decode_chunk(encoded_data, bits),
        17 => CHUNK_DECODER_17.decode_chunk(encoded_data, bits),
        18 => CHUNK_DECODER_18.decode_chunk(encoded_data, bits),
        19 => CHUNK_DECODER_19.decode_chunk(encoded_data, bits),
        20 => CHUNK_DECODER_20.decode_chunk(encoded_data, bits),
        21 => CHUNK_DECODER_21.decode_chunk(encoded_data, bits),
        22 => CHUNK_DECODER_22.decode_chunk(encoded_data, bits),
        23 => CHUNK_DECODER_23.decode_chunk(encoded_data, bits),
        24 => CHUNK_DECODER_24.decode_chunk(encoded_data, bits),
        25 => CHUNK_DECODER_25.decode_chunk(encoded_data, bits),
        26 => CHUNK_DECODER_26.decode_chunk(encoded_data, bits),
        27 => CHUNK_DECODER_27.decode_chunk(encoded_data, bits),
        28 => CHUNK_DECODER_28.decode_chunk(encoded_data, bits),
        29 => CHUNK_DECODER_29.decode_chunk(encoded_data, bits),
        30 => CHUNK_DECODER_30.decode_chunk(encoded_data, bits),
        _ => Err(invalid_ecc_len().into()),
    }
}
