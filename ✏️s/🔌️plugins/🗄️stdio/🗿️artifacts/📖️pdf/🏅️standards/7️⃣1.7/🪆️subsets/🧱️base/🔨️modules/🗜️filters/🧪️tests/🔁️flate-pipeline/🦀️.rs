//! 🔁️ Exercises shared Flate framing recovery and predictor dispatch.

use super::*;

fn bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes().chunks_exact(2).map(|pair| {
        u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()
    }).collect()
}

#[test]
fn content_stream_flate_recovers_framing_then_applies_predictor() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for case in fixture.as_array().unwrap() {
        let encoded = bytes(case["hex"].as_str().unwrap());
        let inflated = bytes(case["inflated"].as_str().unwrap());
        let expected = bytes(case["decoded"].as_str().unwrap());
        assert_eq!(flate_decode(&encoded).unwrap(), inflated, "{}", case["name"]);
        let predictor = case["predictor"].as_i64().unwrap();
        let dictionary = [
            PdfDictEntry::new("Filter", PdfObject::name("FlateDecode")),
            PdfDictEntry::new("DecodeParms", PdfObject::Dict(vec![
                PdfDictEntry::new("Predictor", PdfObject::Int(predictor)),
                PdfDictEntry::new("Columns", PdfObject::Int(3)),
            ])),
        ];
        let (decoded, pipeline) = decode_stream(&dictionary, &encoded).unwrap();
        assert_eq!(decode_stream_admitted(&dictionary, &encoded).unwrap(), (decoded.clone(), pipeline.clone()));
        assert_eq!(decoded, expected, "{}", case["name"]);
        assert_eq!(pipeline, vec![PdfStreamFilter::Flate {
            predictor: (predictor != 1).then_some(PdfPredictor { predictor: predictor as u32, columns: 3, colors: 1, bits_per_component: 8 }),
        }]);
    }
}

#[test]
fn admitted_stream_planning_keeps_typed_flate_and_predictor_declarations() {
    let raw = bytes("789c63e0923362141111010002780098");
    let dictionary = [
        PdfDictEntry::new("F", PdfObject::Array(vec![PdfObject::name("Fl")])),
        PdfDictEntry::new("DP", PdfObject::Array(vec![PdfObject::Dict(vec![
            PdfDictEntry::new("Predictor", PdfObject::Int(15)),
            PdfDictEntry::new("Columns", PdfObject::Int(3)),
        ])])),
    ];
    let source = include_bytes!("../../🪣️stream-data/🧪️tests/🔬️unit/🧪️streams.pdf");
    assert!(source.windows(raw.len()).any(|window| window == raw));
    let expected = PdfStreamFilter::Flate { predictor: Some(PdfPredictor { predictor: 15, columns: 3, colors: 1, bits_per_component: 8 }) };
    let mut cursor = PdfDecodedStreamCursor::new(8, 6);
    let mut grant = StreamDataGrant { work: 1, ..Default::default() };
    for _ in 0..1_000 {
        let receipt = cursor.advance(&dictionary, &raw, grant, false);
        assert!(receipt.work <= grant.work && receipt.allocated_bytes <= grant.allocation_bytes && receipt.released_bytes <= grant.release_bytes);
        let status = receipt.status.unwrap();
        if status == StreamDataStatus::Complete {
            let output = cursor.take_output().unwrap();
            assert_eq!(output.bytes, vec![10, 30, 50, 20, 40, 60]);
            assert_eq!(output.filters, [expected]);
            let grant = StreamDataGrant { work: 1, allocation_bytes: 0, release_bytes: usize::MAX };
            while cursor.close_step(grant).status.unwrap() != StreamDataStatus::Closed {}
            return;
        }
        assert!(cursor.take_output().is_none());
        grant.allocation_bytes = if let StreamDataStatus::NeedsAllocation { bytes } = status { bytes } else { 0 };
        grant.release_bytes = if let StreamDataStatus::NeedsRelease { bytes } = status { bytes } else { 0 };
    }
    panic!("admitted PDF stream did not complete");
}

#[test]
fn admitted_stream_supports_lzw_widths_and_typed_predictor_declarations() {
    let raw = bytes("8000014000a0002901");
    let params = PdfDictEntry::new("DP", PdfObject::Dict(vec![
        PdfDictEntry::new("EarlyChange", PdfObject::Int(0)),
        PdfDictEntry::new("Predictor", PdfObject::Int(2)),
        PdfDictEntry::new("Columns", PdfObject::Int(3)),
        PdfDictEntry::new("BitsPerComponent", PdfObject::Int(16)),
    ]));
    let dictionary = [PdfDictEntry::new("F", PdfObject::name("LZW")), params];
    let expected = vec![0, 10, 0, 30, 0, 50];
    let pipeline = vec![PdfStreamFilter::Lzw {
        early_change: false, predictor: Some(PdfPredictor { predictor: 2, columns: 3, colors: 1, bits_per_component: 16 }),
    }];
    let mut cursor = PdfDecodedStreamCursor::new(6, 6);
    let mut grant = StreamDataGrant { work: 1, ..Default::default() };
    let mut complete = false;
    for _ in 0..1000 {
        let receipt = cursor.advance(&dictionary, &raw, grant, false);
        assert!(receipt.work <= grant.work);
        let status = receipt.status.unwrap();
        if status == StreamDataStatus::Complete {
            let output = cursor.take_output().unwrap();
            assert_eq!(output.bytes, expected);
            assert_eq!(output.filters, pipeline);
            complete = true;
            break;
        }
        assert!(cursor.take_output().is_none());
        grant.allocation_bytes = if let StreamDataStatus::NeedsAllocation { bytes } = status { bytes } else { 0 };
        grant.release_bytes = if let StreamDataStatus::NeedsRelease { bytes } = status { bytes } else { 0 };
    }
    assert!(complete);
    let close = StreamDataGrant { work: 1, allocation_bytes: 0, release_bytes: usize::MAX };
    while cursor.close_step(close).status.unwrap() != StreamDataStatus::Closed {}
    assert_eq!(decode_stream_admitted(&dictionary, &raw).unwrap(), (expected, pipeline));

    let mut random = 7u32;
    let data = (0..16_384).map(|_| {
        random = random.wrapping_mul(1664525).wrapping_add(1013904223);
        (random >> 24) as u8
    }).collect::<Vec<_>>();
    for early in [false, true] {
        let encoded = lzw_encode(&data, early);
        assert_eq!(lzw_decode(&encoded, early).unwrap(), data);
    }
}

#[test]
fn admitted_filter_chain_preserves_order_parameters_and_intermediate_custody() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let raw = fixture[0]["chain"].as_str().unwrap().as_bytes();
    let expected = bytes(fixture[0]["decoded"].as_str().unwrap());
    let filters = ["ASCIIHexDecode", "ASCII85Decode", "FlateDecode"].into_iter().map(PdfObject::name).collect();
    let dictionary = [
        PdfDictEntry::new("Filter", PdfObject::Array(filters)),
        PdfDictEntry::new("DecodeParms", PdfObject::Array(vec![
            PdfObject::Null, PdfObject::Null, PdfObject::Dict(vec![
                PdfDictEntry::new("Predictor", PdfObject::Int(15)),
                PdfDictEntry::new("Columns", PdfObject::Int(3)),
            ]),
        ])),
    ];
    let pipeline = vec![
        PdfStreamFilter::AsciiHex, PdfStreamFilter::Ascii85,
        PdfStreamFilter::Flate { predictor: Some(PdfPredictor { predictor: 15, columns: 3, colors: 1, bits_per_component: 8 }) },
    ];
    assert_eq!(decode_stream_admitted(&dictionary, raw).unwrap(), (expected.clone(), pipeline));

    for cancelled in [false, true] {
        let mut cursor = PdfDecodedStreamCursor::new(64, 6);
        let mut grant = StreamDataGrant { work: 1, ..Default::default() };
        let mut counts = (0, 0);
        let mut allocated = 0;
        let mut released = 0;
        let mut finished = None;
        for _ in 0..20_000 {
            let cancel_now = cancelled && counts.0 == raw.len();
            let receipt = cursor.advance(&dictionary, raw, grant, cancel_now);
            assert!(receipt.work <= grant.work && receipt.allocated_bytes <= grant.allocation_bytes && receipt.released_bytes <= grant.release_bytes);
            allocated += receipt.allocated_bytes;
            released += receipt.released_bytes;
            let progress = cursor.progress();
            assert!(progress.0 >= counts.0 && progress.1 >= counts.1);
            counts = progress;
            let status = receipt.status.unwrap();
            if matches!(status, StreamDataStatus::Complete | StreamDataStatus::Cancelled) {
                finished = Some(status);
                break;
            }
            assert!(cursor.take_output().is_none());
            grant.allocation_bytes = if let StreamDataStatus::NeedsAllocation { bytes } = status { bytes } else { 0 };
            grant.release_bytes = if let StreamDataStatus::NeedsRelease { bytes } = status { bytes } else { 0 };
        }
        let owned = if cancelled {
            assert_eq!(finished, Some(StreamDataStatus::Cancelled));
            assert!(cursor.take_bytes().is_none());
            0
        } else {
            assert_eq!(finished, Some(StreamDataStatus::Complete));
            let output = cursor.take_bytes().unwrap();
            assert_eq!(output, expected);
            assert!(cursor.take_output().is_none());
            output.capacity()
        };
        cursor.begin_close();
        let mut closed = false;
        let mut grant = StreamDataGrant { work: 1, ..Default::default() };
        for _ in 0..100 {
            let receipt = cursor.close_step(grant);
            assert!(receipt.work <= grant.work && receipt.released_bytes <= grant.release_bytes);
            released += receipt.released_bytes;
            let status = receipt.status.unwrap();
            if status == StreamDataStatus::Closed { closed = true; break; }
            grant.release_bytes = if let StreamDataStatus::NeedsRelease { bytes } = status { bytes } else { 0 };
        }
        assert!(closed);
        assert_eq!(allocated, released + owned);
    }
}

#[test]
fn unsupported_pipeline_is_detected_before_decoding_supported_prefixes() {
    let filters = [PdfObject::name("ASCIIHexDecode"), PdfObject::name("DCTDecode")];
    let dictionary = [PdfDictEntry::new("Filter", PdfObject::Array(filters.into()))];
    let raw = vec![b'1'; 16 * 1024];
    let grant = StreamDataGrant { work: 1, allocation_bytes: 256 * 1024, release_bytes: 256 * 1024 };
    let mut cursor = PdfDecodedStreamCursor::new(raw.len(), raw.len());
    let mut refused = false;
    for _ in 0..100 {
        let step = cursor.advance(&dictionary, &raw, grant, false);
        assert!(step.work <= 1);
        assert_eq!(step.allocated_bytes, 0);
        assert_eq!(cursor.progress(), (0, 0));
        if let Err(error) = step.status {
            assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::UnsupportedOwner);
            refused = true;
            break;
        }
    }
    assert!(refused);
    cursor.begin_close();
    assert_eq!(cursor.close_step(grant).status.unwrap(), StreamDataStatus::Closed);
}
