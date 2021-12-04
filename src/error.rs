use core::fmt::{Debug, Display, Formatter};

enum UsageErrorType {
    // Decoder or Encoder errors
    InvalidECCLen,
    InvalidBits,

    // Encoder errors
    EncodeBufferTooBig,
    EncodeBufferDoesntMatchBits,
    TotalEncodeLenTooLong,

    // Decoder errors
    DecodeBufferTooBig,
    DecodeBufferSmallerThanEcc,
    DecodeBufferWrongSize,
}

/// A `UsageErrorCause` can be formatted to get a string
/// explaining the error.
pub struct UsageErrorCause {
    typ: UsageErrorType,
}

impl Debug for UsageErrorCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.typ {
            UsageErrorType::InvalidECCLen => write!(f, "The number of error correcting symbols must be in the range [0,30]"),
            UsageErrorType::InvalidBits => write!(f, "The number of bits to process must be in the range [1,150]"),
            UsageErrorType::EncodeBufferTooBig => write!(f, "The buffer to encode must be no larger than 19 bytes (up to 150 bits of that can be encoded)"),
            UsageErrorType::EncodeBufferDoesntMatchBits => write!(f, "The size of the encode buffer didn't match the bits parameter"),
            UsageErrorType::TotalEncodeLenTooLong => write!(f, "The size of encoded data after adding ECC symbols would exceed 31 characters"),
            UsageErrorType::DecodeBufferTooBig => write!(f, "The buffer to decode contained more than 31 encoded characters"),
            UsageErrorType::DecodeBufferSmallerThanEcc => write!(f, "The buffer to decode was smaller than the number of ECC symbols"),
            UsageErrorType::DecodeBufferWrongSize => write!(f, "The size of the decode buffer didn't match the bits parameter"),
        }
    }
}

impl Display for UsageErrorCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

/// Error type used by decode operations
///
/// By design, this type is mostly opaque - with the exception
/// that its possible to differentiate between the input being
/// invalid (having too many errors) and using the API incorrectly.
/// The [`Debug`] or [`Display`] implementations can be used to
/// format a more specific error message.
pub enum DecodeError {
    /// The input had too many errors and couldn't be decoded
    TooManyErrors,

    /// A UsageError indicates an error outside of an invalid input value.
    UsageError(UsageErrorCause),
}

impl Debug for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            DecodeError::TooManyErrors => write!(f, "There were too many errors in the input"),
            DecodeError::UsageError(cause) => write!(f, "Usage Error: {}", cause),
        }
    }
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeError {}

impl From<UsageError> for DecodeError {
    fn from(UsageError(cause): UsageError) -> Self {
        DecodeError::UsageError(cause)
    }
}

/// Error type to indicate that an API was used incorrectly
///
/// By design, this type is mostly opaque. The [`Debug`] or [`Display`]
/// implementations can be used to format a more specific error
/// message.
pub struct UsageError(pub UsageErrorCause);

impl Debug for UsageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for UsageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UsageError {}

pub const fn too_many_errors() -> DecodeError {
    DecodeError::TooManyErrors
}

pub const fn invalid_ecc_len() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::InvalidECCLen,
    })
}

pub const fn invalid_bits() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::InvalidBits,
    })
}

pub const fn encode_buffer_too_big() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::EncodeBufferTooBig,
    })
}

pub const fn encode_buffer_doesnt_match_bits() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::EncodeBufferDoesntMatchBits,
    })
}

pub const fn total_encode_len_too_long() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::TotalEncodeLenTooLong,
    })
}

pub const fn decode_buffer_too_big() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::DecodeBufferTooBig,
    })
}

pub const fn decode_buffer_smaller_than_ecc() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::DecodeBufferSmallerThanEcc,
    })
}

pub const fn decode_buffer_wrong_size() -> UsageError {
    UsageError(UsageErrorCause {
        typ: UsageErrorType::DecodeBufferWrongSize,
    })
}
