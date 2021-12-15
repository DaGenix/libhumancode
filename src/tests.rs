use crate::{decode_chunk, encode_chunk};

#[test]
fn test_happy_path() {
    const CODE: &'static str = "urs7wdsq9jkyoxu8oxrf16kj7o16qb5";
    const VALUE: &'static [u8] = &[
        153, 45, 218, 14, 206, 250, 84, 8, 62, 103, 131, 200, 89, 121, 73, 236,
    ];

    let encoded = encode_chunk(VALUE, 128, 5).unwrap().raw();
    assert_eq!(encoded.as_str(), CODE);

    let decode_output = decode_chunk(encoded.as_str(), 128, 5).unwrap();
    assert_eq!(decode_output.data(), VALUE);
}

#[test]
fn test_erasures() {
    const GOOD_CODE: &'static str = "urs7wdsq9jkyoxu8oxrf16kj7o16qb5";
    // Same code as the GOOD_CODE, but 5 invalid "2"s were added
    const BAD_CODE: &'static str = "urs72dsq9j2yoxu2oxrf16kj7o26qb2";
    const VALUE: &'static [u8] = &[
        153, 45, 218, 14, 206, 250, 84, 8, 62, 103, 131, 200, 89, 121, 73, 236,
    ];

    let encoded = encode_chunk(VALUE, 128, 5).unwrap().raw();
    assert_eq!(encoded.as_str(), GOOD_CODE);

    let decode_output = decode_chunk(BAD_CODE, 128, 5).unwrap();
    assert_eq!(decode_output.data(), VALUE);
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

    let encoded = encode_chunk(VALUE, 128, 5).unwrap().raw();
    assert_eq!(encoded.as_str(), GOOD_CODE);

    let decode_output = decode_chunk(BAD_CODE, 128, 5).unwrap();
    assert_eq!(decode_output.data(), VALUE);
}

#[test]
fn test_encode_all_length_combos() {
    for data_bits_len in 0..=156 {
        for ecc_len in 0..=32 {
            let data = [0x80; 31];
            let data = &data[0..(data_bits_len + 7) / 8];

            let expected_ok =
                data_bits_len + ecc_len * 5 <= 155 && ecc_len < 31 && data_bits_len > 0;

            let encode_result = encode_chunk(data, data_bits_len as u8, ecc_len as u8);

            assert_eq!(expected_ok, encode_result.is_ok());

            if expected_ok {
                let decode_result = decode_chunk(
                    &encode_result.unwrap().pretty().as_str(),
                    data_bits_len as u8,
                    ecc_len as u8,
                );

                assert!(decode_result.is_ok());
                assert_eq!(&data, &decode_result.unwrap().data());
            }
        }
    }
}

#[test]
fn decode_edgecase_1() {
    let encoded = "yyyy-yyyy-yyyy-yyyy-yyyy-yyyy-yyyy-xxx";
    let result = decode_chunk(encoded, 1, 30);
    assert!(result.is_ok());
}

#[test]
fn decode_edgecase_2() {
    let encoded = "yyyy-yyyy-yyyy-yyyy-yyyy-yyyy-yyyy-xxx";
    let result = decode_chunk(encoded, 140, 3);
    assert!(result.is_ok());
}

#[test]
fn encode_failure_with_nonzero_trailing_bits() {
    let data = [0xff; 4];
    let result = encode_chunk(&data, 31, 4);
    assert!(result.is_err())
}
