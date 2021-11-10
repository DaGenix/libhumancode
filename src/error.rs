use core::fmt::{Debug, Display, Formatter};

/// Common error type used by all fallible operations
///
/// By design, this type is mostly opaque - with the exception
/// that its possible to differentiate between errors with input
/// data and other types of errors. The [`Debug`] or [`Display`]
/// implementations can be used to format a more specific error
/// message.
pub struct HumancodeError {
    error_info: HumancodeErrorInfo,
}

/// Provides a set of error categories for HumancodeError values
pub enum HumancodeErrorType {
    /// An InputError indicates that an input array contained an invalid
    /// value. For example, an invalid character being passed to
    /// the [`decode_chunk`](crate::ChunkDecoder::decode_chunk) method.
    InputError,

    /// A UsageError indicates an error outside of an invalid input value.
    UsageError,
}

pub enum HumancodeErrorInfo {
    // Input errors
    TooManyErrors,

    // Decoder or Encoder errors
    InvalidECCLen,
    InvalidBits,

    // Encoder errors
    EncodeBufferTooBig,
    EncodeBufferDoesntMatchBits,
    TotalEncodeLenTooLong,

    // Decoder errors
    DecoderBufferTooBig,
    DecodeBufferSmallerThanEcc,
    DecodeBufferWrongSize,
}

impl HumancodeError {
    /// Get the type of the error
    ///
    /// The type is either [`HumancodeErrorType::InputError`] to
    /// indicate that something was wrong with the input or
    /// [`HumancodeErrorType::UsageError`] to indicate that an API
    /// was used incorrectly.
    pub fn error_type(&self) -> HumancodeErrorType {
        match self.error_info {
            HumancodeErrorInfo::TooManyErrors => HumancodeErrorType::InputError,
            HumancodeErrorInfo::InvalidECCLen => HumancodeErrorType::UsageError,
            HumancodeErrorInfo::InvalidBits => HumancodeErrorType::UsageError,
            HumancodeErrorInfo::EncodeBufferTooBig => HumancodeErrorType::UsageError,
            HumancodeErrorInfo::EncodeBufferDoesntMatchBits => HumancodeErrorType::UsageError,
            HumancodeErrorInfo::TotalEncodeLenTooLong => HumancodeErrorType::UsageError,
            HumancodeErrorInfo::DecoderBufferTooBig => HumancodeErrorType::UsageError,
            HumancodeErrorInfo::DecodeBufferSmallerThanEcc => HumancodeErrorType::UsageError,
            HumancodeErrorInfo::DecodeBufferWrongSize => HumancodeErrorType::UsageError,
        }
    }
}

impl Debug for HumancodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.error_info {
            HumancodeErrorInfo::TooManyErrors => write!(f, "There were too many errors in the data to decode"),
            HumancodeErrorInfo::InvalidECCLen => write!(f, "The number of error correcting symbols must be in the range [1,30]"),
            HumancodeErrorInfo::InvalidBits => write!(f, "The number of bits to process must be in the range [1,150]"),
            HumancodeErrorInfo::EncodeBufferTooBig => write!(f, "The buffer to encode must be no larger than 19 bytes (up to 150 bits of that can be encoded)"),
            HumancodeErrorInfo::EncodeBufferDoesntMatchBits => write!(f, "The size of the encode buffer didn't match the bits parameter"),
            HumancodeErrorInfo::TotalEncodeLenTooLong => write!(f, "The size of encoded data after adding ECC symbols would exceed 31 characters"),
            HumancodeErrorInfo::DecoderBufferTooBig => write!(f, "The buffer to decode contained more than 31 encoded characters"),
            HumancodeErrorInfo::DecodeBufferSmallerThanEcc => write!(f, "The buffer to decode was smaller than the number of ECC symbols"),
            HumancodeErrorInfo::DecodeBufferWrongSize => write!(f, "The size of the decode buffer didn't match the bits parameter"),
        }
    }
}

impl Display for HumancodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HumancodeError {}

pub fn error(error_info: HumancodeErrorInfo) -> HumancodeError {
    HumancodeError { error_info }
}
