//#region 🧪️ContentFraming
use super::return_content::{ReturnContentHeader, ReturnContentHeaderReader};

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap() }
fn unhex(value: &str) -> Vec<u8> { (0..value.len()).step_by(2).map(|offset| u8::from_str_radix(&value[offset..offset + 2], 16).unwrap()).collect() }

#[test]
fn return_content_framing_header_matches_neutral_records_at_each_byte_grant() {
    let fixture = fixture();
    for row in fixture["recordVectors"].as_array().unwrap() {
        let frame = unhex(row["frameHex"].as_str().unwrap());
        let body = unhex(row["bodyHex"].as_str().unwrap());
        let expected = &frame[..frame.len() - body.len()];
        for grant in [1, 64, 4096] {
            let mut cursor = ReturnContentHeader::new(frame[0], body.len() as u64).unwrap();
            let mut result = Vec::new();
            for _ in 0..11 {
                let mut output = [73; 32];
                let step = cursor.write(&mut output, 0, grant);
                assert_eq!(step.written_bytes, 0);
                assert_eq!(output, [73; 32]);
                let step = cursor.write(&mut output, 1, 0);
                assert_eq!(step.written_bytes, 0);
                assert_eq!(output, [73; 32]);
                let step = cursor.write(&mut output, 1, grant);
                assert!(step.written_bytes <= grant);
                assert!(output[step.written_bytes..].iter().all(|byte| *byte == 73));
                result.extend_from_slice(&output[..step.written_bytes]);
                if step.complete { break; }
            }
            assert_eq!(result, expected);
            assert!(cursor.is_complete());
        }
    }
    let mut maximum = ReturnContentHeader::new(5, u64::MAX).unwrap();
    let mut output = [0; 11];
    assert_eq!(maximum.write(&mut output, 1, 11).written_bytes, 11);
    assert_eq!(output, [5, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
    assert!(ReturnContentHeader::new(10, 0).is_err());
}

#[test]
fn return_content_framing_reader_owns_split_prefix_without_consuming_body() {
    for tag in 0..10 {
        for length in [0, 1, 127, 128, 4096, u64::MAX] {
            let mut writer = ReturnContentHeader::new(tag, length).unwrap();
            let mut bytes = [0; 11];
            let count = writer.write(&mut bytes, 1, 11).written_bytes;
            for split in 0..=count {
                let mut reader = ReturnContentHeaderReader::new();
                assert_eq!(reader.consume(&bytes[..split], 0, 11).unwrap().consumed_bytes, 0);
                assert_eq!(reader.consume(&bytes[..split], 1, 0).unwrap().consumed_bytes, 0);
                assert_eq!(reader.consume(&bytes[..split], 1, 11).unwrap().consumed_bytes, split);
                if split < count { assert!(reader.finish().is_err()); }
                let mut suffix = bytes[split..count].to_vec();
                suffix.extend_from_slice(&[23, 29]);
                assert_eq!(reader.consume(&suffix, 1, 11).unwrap().consumed_bytes, count - split);
                assert_eq!(reader.value(), Some((tag, length)));
                assert_eq!(reader.finish().unwrap(), (tag, length));
                assert_eq!(reader.consume(&[31], 1, 1).unwrap().consumed_bytes, 0);
            }
        }
    }
    for invalid in [&[10][..], &[0, 128, 0], &[0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 2], &[0, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128]] {
        let mut reader = ReturnContentHeaderReader::new();
        let error = reader.consume(invalid, 1, 11).unwrap_err();
        assert_eq!(error.consumed_bytes, invalid.len());
        assert_eq!(reader.value(), None);
        assert_eq!(reader.consume(&[0, 0], 1, 11).unwrap_err().consumed_bytes, 0);
    }
}
//#endregion 🧪️ContentFraming
