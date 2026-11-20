//! 🧩️ Exercises PDF Flate framing across tiny grants and the retained close protocol.

use super::*;

fn fixtures() -> serde_json::Value {
    serde_json::from_str(include_str!("🔣️.json")).unwrap()
}

pub(super) fn bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes().chunks_exact(2).map(|pair| {
        u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()
    }).collect()
}

fn retire(mut cursor: PdfFlateCursor) {
    cursor.begin_close();
    let mut grants = 0;
    let mut released = 0;
    let retained = cursor.owned_bytes();
    while !cursor.terminal_is_empty() {
        assert!(grants < 8);
        let demand = cursor.retirement_demand();
        let refused = cursor.close_step(0, 0);
        assert_eq!(refused.released_items + refused.released_bytes + refused.work, 0);
        if demand.release_bytes > 0 {
            let refused = cursor.close_step(1, demand.release_bytes - 1);
            assert_eq!(refused.released_items + refused.released_bytes + refused.work, 0);
        }
        let step = cursor.close_step(1, demand.release_bytes);
        assert!(step.work <= 1 && step.released_items <= 1 && step.released_bytes <= demand.release_bytes);
        released += step.released_bytes;
        grants += 1;
    }
    assert_eq!(retained, released);
}

#[test]
fn content_stream_flate_resumes_without_claiming_flat_output() {
    for case in fixtures().as_array().unwrap() {
        let source = bytes(case["hex"].as_str().unwrap());
        let expected = case["unit"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap() as usize);
        let mut cursor = PdfFlateCursor::new(expected.len());
        let idle = cursor.advance(&source, &mut [0; 7], FlateGrant::default(), false);
        assert_eq!(idle.status.unwrap(), FlateStatus::Yield);
        assert_eq!(cursor.progress(), (0, 0));
        let initial = cursor.advance(&source, &mut [0; 7], FlateGrant { work: 2, allocation_bytes: 0 }, false);
        let FlateStatus::NeedsHistory { bytes } = initial.status.unwrap() else { panic!("expected retained history admission") };
        assert_eq!(initial.work, 1);
        assert_eq!(initial.allocated_bytes, 0);
        let mut decoded = Vec::new();
        let mut last = cursor.progress();
        loop {
            let mut page = [0; 7];
            let grant = FlateGrant { work: 11, allocation_bytes: bytes };
            let turn = cursor.advance(&source, &mut page, grant, false);
            assert!(turn.work <= grant.work && turn.allocated_bytes <= grant.allocation_bytes && turn.written_bytes <= page.len());
            decoded.extend_from_slice(&page[..turn.written_bytes]);
            let current = cursor.progress();
            assert!(current.0 >= last.0 && current.1 >= last.1);
            last = current;
            match turn.status.unwrap() {
                FlateStatus::Complete => break,
                FlateStatus::Yield | FlateStatus::NeedsOutput => {}
                unexpected => panic!("unexpected Flate status {unexpected:?}"),
            }
            assert!(turn.work > 0);
        }
        assert_eq!(last, (source.len(), expected.len()));
        retire(cursor);
        assert_eq!(decoded, expected.as_bytes());
    }
}

#[test]
fn content_stream_flate_accounts_for_failure_and_cancellation() {
    let case = &fixtures()[1];
    let mut source = bytes(case["hex"].as_str().unwrap());
    let expected = case["unit"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap() as usize);
    let grant = FlateGrant { work: 1 << 20, allocation_bytes: 1 << 15 };
    let mut small = PdfFlateCursor::new(10);
    let mut output = [0; 16];
    let failed = small.advance(&source, &mut output, grant, false);
    assert!(failed.status.is_err());
    assert_eq!(failed.written_bytes, 10);
    assert!(failed.work > 10);
    retire(small);
    let mut cancelled = PdfFlateCursor::new(expected.len());
    let first = cancelled.advance(&source, &mut output, FlateGrant { work: 17, ..grant }, false);
    assert!(first.status.is_ok());
    let before = cancelled.progress();
    let stop = cancelled.advance(&source, &mut output, grant, true);
    assert_eq!(stop.status.unwrap(), FlateStatus::Cancelled);
    assert_eq!(stop.work + stop.written_bytes + stop.allocated_bytes, 0);
    assert_eq!(cancelled.progress(), before);
    retire(cancelled);
    *source.last_mut().unwrap() ^= 1;
    let mut checksum = PdfFlateCursor::new(expected.len());
    let mut output = vec![0; expected.len()];
    let failed = checksum.advance(&source, &mut output, grant, false);
    assert!(failed.status.is_err());
    assert_eq!(failed.written_bytes, expected.len());
    retire(checksum);
}

#[test]
fn content_stream_flate_refuses_truncation_and_trailing_deflate_data() {
    let case = &fixtures()[0];
    let source = bytes(case["hex"].as_str().unwrap());
    let expected = case["unit"].as_str().unwrap().len();
    let mut trailing = source.clone();
    trailing.insert(trailing.len() - 4, 0);
    for invalid in [&source[..source.len() - 3], &trailing] {
        let mut cursor = PdfFlateCursor::new(expected);
        let grant = FlateGrant { work: 256, allocation_bytes: 32 * 1024 };
        let turn = cursor.advance(invalid, &mut [0; 64], grant, false);
        assert!(turn.status.is_err());
        assert!(turn.work > 0 && turn.work <= grant.work);
        retire(cursor);
    }
}


#[test]
fn content_stream_flate_preserves_raw_recovery() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🔁️flate-pipeline/🔣️.json")).unwrap();
    for case in fixtures.as_array().unwrap() {
        let encoded = bytes(case["hex"].as_str().unwrap());
        let start = usize::from(case["framing"] == "leading") * 3;
        let inflated = super::bounded_flate::decode_compatible(&encoded[start..]).unwrap();
        assert_eq!(inflated, bytes(case["inflated"].as_str().unwrap()), "{}", case["name"]);
    }
}
