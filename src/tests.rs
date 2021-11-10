use crate::{ChunkDecoder, ChunkEncoder};

#[test]
fn test_happy_path() {
    const CODE: &'static str = "urs7wdsq9jkyoxu8oxrf16kj7o16qb5";
    const VALUE: &'static [u8] = &[
        153, 45, 218, 14, 206, 250, 84, 8, 62, 103, 131, 200, 89, 121, 73, 236,
    ];

    let encoded = ChunkEncoder::new(5)
        .unwrap()
        .encode_chunk(VALUE, 128)
        .unwrap()
        .raw();
    assert_eq!(encoded.as_str(), CODE);

    let (decoded, _) = ChunkDecoder::new(5)
        .unwrap()
        .decode_chunk(encoded.as_str(), 128)
        .unwrap();
    assert_eq!(decoded.as_bytes(), VALUE);
}

#[test]
fn test_erasures() {
    const GOOD_CODE: &'static str = "urs7wdsq9jkyoxu8oxrf16kj7o16qb5";
    // Same code as the GOOD_CODE, but 5 invalid "2"s were added
    const BAD_CODE: &'static str = "urs72dsq9j2yoxu2oxrf16kj7o26qb2";
    const VALUE: &'static [u8] = &[
        153, 45, 218, 14, 206, 250, 84, 8, 62, 103, 131, 200, 89, 121, 73, 236,
    ];

    let encoded = ChunkEncoder::new(5)
        .unwrap()
        .encode_chunk(VALUE, 128)
        .unwrap()
        .raw();
    assert_eq!(encoded.as_str(), GOOD_CODE);

    let (decoded, _) = ChunkDecoder::new(5)
        .unwrap()
        .decode_chunk(BAD_CODE, 128)
        .unwrap();
    assert_eq!(decoded.as_bytes(), VALUE);
}

#[test]
fn test_invalid_trailing_octet() {
    const GOOD_CODE: &'static str = "urs7wdsq9jkyoxu8oxrf16kj7o16qb5";
    // Same code as the GOOD_CODE, but 3 invalid "2"s were added and the
    // final octet was changed to a "9" - which is invalid for 128 bits
    const BAD_CODE: &'static str = "urs7-wdsq-9jky-oxu8-oxrf-16kj-7912-222";
    const VALUE: &'static [u8] = &[
        153, 45, 218, 14, 206, 250, 84, 8, 62, 103, 131, 200, 89, 121, 73, 236,
    ];

    let encoded = ChunkEncoder::new(5)
        .unwrap()
        .encode_chunk(VALUE, 128)
        .unwrap()
        .raw();
    assert_eq!(encoded.as_str(), GOOD_CODE);

    let (decoded, _) = ChunkDecoder::new(5)
        .unwrap()
        .decode_chunk(BAD_CODE, 128)
        .unwrap();
    assert_eq!(decoded.as_bytes(), VALUE);
}
