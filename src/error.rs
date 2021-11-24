use core::fmt::{Debug, Display, Formatter};

/// An `InputErrorCause` can be formatted to get a string
/// explaining the error.
///
/// Currently, there is only one input error message
/// (that there were too many errors) but more could be added
/// in the future.
pub struct InputErrorCause {
    __hidden: (),
}

impl Debug for InputErrorCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "There were too many errors in the data to decode")
    }
}

impl Display for InputErrorCause {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

/// Common error type used by all fallible operations
///
/// By design, this type is mostly opaque - with the exception
/// that its possible to differentiate between errors with input
/// data and other types of errors. The [`Debug`] or [`Display`]
/// implementations can be used to format a more specific error
/// message.
pub enum HumancodeError {
    /// An InputError indicates that an input array contained an invalid
    /// value. For example, an invalid character being passed to
    /// the [`decode_chunk`](crate::decode_chunk()) method.
    InputError(InputErrorCause),

    /// A UsageError indicates an error outside of an invalid input value.
    UsageError(UsageErrorCause),
}

impl Debug for HumancodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            HumancodeError::InputError(cause) => write!(f, "Input Error: {}", cause),
            HumancodeError::UsageError(cause) => write!(f, "Usage Error: {}", cause),
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

pub const fn too_many_errors() -> HumancodeError {
    HumancodeError::InputError(InputErrorCause { __hidden: () })
}

pub const fn invalid_ecc_len() -> HumancodeError {
    HumancodeError::UsageError(UsageErrorCause {
        typ: UsageErrorType::InvalidECCLen,
    })
}

pub const fn invalid_bits() -> HumancodeError {
    HumancodeError::UsageError(UsageErrorCause {
        typ: UsageErrorType::InvalidBits,
    })
}

pub const fn encode_buffer_too_big() -> HumancodeError {
    HumancodeError::UsageError(UsageErrorCause {
        typ: UsageErrorType::EncodeBufferTooBig,
    })
}

pub const fn encode_buffer_doesnt_match_bits() -> HumancodeError {
    HumancodeError::UsageError(UsageErrorCause {
        typ: UsageErrorType::EncodeBufferDoesntMatchBits,
    })
}

pub const fn total_encode_len_too_long() -> HumancodeError {
    HumancodeError::UsageError(UsageErrorCause {
        typ: UsageErrorType::TotalEncodeLenTooLong,
    })
}

pub const fn decode_buffer_too_big() -> HumancodeError {
    HumancodeError::UsageError(UsageErrorCause {
        typ: UsageErrorType::DecodeBufferTooBig,
    })
}

pub const fn decode_buffer_smaller_than_ecc() -> HumancodeError {
    HumancodeError::UsageError(UsageErrorCause {
        typ: UsageErrorType::DecodeBufferSmallerThanEcc,
    })
}

pub const fn decode_buffer_wrong_size() -> HumancodeError {
    HumancodeError::UsageError(UsageErrorCause {
        typ: UsageErrorType::DecodeBufferWrongSize,
    })
}
