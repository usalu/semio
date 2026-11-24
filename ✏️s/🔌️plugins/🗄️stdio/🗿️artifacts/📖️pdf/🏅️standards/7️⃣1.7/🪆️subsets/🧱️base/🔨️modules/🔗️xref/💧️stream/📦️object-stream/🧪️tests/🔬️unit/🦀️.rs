//! 🧪 Compressed object stream header and typed members from a genuine PDF.

use super::*;
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters::decode_stream_admitted;
use semio_s_artifact_stdio_pdf_lexer::parse_indirect_at;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::ObjRef;

const SOURCE: &[u8] = include_bytes!("../../../../../../🧪️tests/🧫️fixtures/📦️object-stream/🧪️compressed.pdf");

fn decoded_fixture() -> Vec<u8> {
    let offset = SOURCE.windows(8).position(|bytes| bytes == b"4 0 obj\n").unwrap();
    let (_, PdfObject::Stream { dict, data, .. }) = parse_indirect_at(SOURCE, offset).unwrap() else { panic!("fixture object stream"); };
    decode_stream_admitted(&dict, &data.to_vec_owner()).unwrap().0
}

fn member(decoded: &[u8], start: usize, end: usize) -> PdfObject {
    let mut cursor = ObjectStreamMemberCursor::new(start, end, CosLimits::default());
    let mut grant = CosGrant { work: 1, ..Default::default() };
    for _ in 0..1024 {
        let receipt = cursor.advance(decoded, grant, false);
        assert!(receipt.work <= grant.work && receipt.allocated_bytes <= grant.allocation_bytes && receipt.released_bytes <= grant.release_bytes);
        let status = receipt.status.unwrap();
        if let CosStatus::Complete { end: finished } = status {
            assert_eq!(finished, end);
            return cursor.take_value().unwrap();
        }
        assert!(cursor.take_value().is_none());
        grant.allocation_bytes = if let CosStatus::NeedsAllocation { bytes } = status { bytes } else { 0 };
        grant.release_bytes = if let CosStatus::NeedsRelease { bytes } = status { bytes } else { 0 };
    }
    panic!("object stream member did not finish");
}

#[test]
fn object_stream_advances_header_and_native_members_one_work_unit_at_a_time() {
    let decoded = decoded_fixture();
    let mut cursor = ObjectStreamHeaderCursor::new(2, 26);
    let mut pairs = Vec::new();
    for _ in 0..256 {
        let receipt = cursor.advance(&decoded, 1, false);
        assert!(receipt.work <= 1);
        match receipt.event.unwrap() {
            ObjectStreamHeaderEvent::Member { number, offset } => pairs.push((number, offset)),
            ObjectStreamHeaderEvent::More => {}
            ObjectStreamHeaderEvent::Complete => break,
            ObjectStreamHeaderEvent::Cancelled => panic!("unexpected cancellation"),
        }
    }
    assert_eq!(pairs, [(6, 26), (7, 70)]);
    let first = member(&decoded, pairs[0].1, pairs[1].1);
    assert_eq!(first.dict_get("Title").unwrap().as_str_bytes(), Some(b"Compressed".as_slice()));
    let member = member(&decoded, pairs[1].1, decoded.len());
    assert_eq!(member.as_array().unwrap(), [
        PdfObject::Bool(true), PdfObject::Name("Object".into()), PdfObject::Str(b"packed".to_vec()),
        PdfObject::Ref(ObjRef { num: 6, gen: 0 }),
    ]);
}

#[test]
fn object_stream_member_cancellation_retires_staged_native_values() {
    let decoded = decoded_fixture();
    let mut header = ObjectStreamHeaderCursor::new(2, 26);
    assert_eq!(header.advance(&decoded, 10, false).event, Ok(ObjectStreamHeaderEvent::More));
    let closed = header.advance(&decoded, 10, true);
    assert_eq!(closed, ObjectStreamHeaderReceipt { event: Ok(ObjectStreamHeaderEvent::Cancelled), work: 0 });

    let mut cursor = ObjectStreamMemberCursor::new(26, 70, CosLimits::default());
    let mut retained = 0;
    let mut grant = CosGrant { work: 1, allocation_bytes: 4096, release_bytes: 4096 };
    for _ in 0..34 {
        let receipt = cursor.advance(&decoded, grant, false);
        assert!(receipt.status.is_ok());
        retained += receipt.allocated_bytes;
        retained -= receipt.released_bytes;
    }
    assert!(retained > 0);
    let stop = cursor.advance(&decoded, grant, true);
    assert_eq!(stop.status, Ok(CosStatus::Cancelled));
    assert_eq!(stop.work + stop.allocated_bytes + stop.released_bytes, 0);
    assert!(cursor.take_value().is_none());
    cursor.begin_close();
    grant.allocation_bytes = 0;
    grant.release_bytes = 0;
    for _ in 0..256 {
        let receipt = cursor.close_step(grant);
        assert!(receipt.work <= grant.work && receipt.released_bytes <= grant.release_bytes);
        retained -= receipt.released_bytes;
        match receipt.status.unwrap() {
            CosStatus::Closed => { assert_eq!(retained, 0); return; }
            CosStatus::NeedsRelease { bytes } => grant.release_bytes = bytes,
            _ => grant.release_bytes = 0,
        }
    }
    panic!("object stream member did not retire");
}

fn controlled_step(cursor: &mut ObjectStreamCursor, source: &[u8], grant: &mut CosGrant, retained: &mut usize, cancelled: bool) -> Result<ObjectStreamEvent, &'static str> {
    let step = cursor.advance(source, *grant, cancelled);
    assert!(step.work <= grant.work && step.allocated_bytes <= grant.allocation_bytes && step.released_bytes <= grant.release_bytes);
    *retained += step.allocated_bytes;
    *retained -= step.released_bytes;
    grant.allocation_bytes = if let Ok(ObjectStreamEvent::NeedsAllocation { bytes }) = step.event { bytes } else { 0 };
    grant.release_bytes = if let Ok(ObjectStreamEvent::NeedsRelease { bytes }) = step.event { bytes } else { 0 };
    step.event
}

fn selected_value(cursor: &mut ObjectStreamCursor, source: &[u8], end: usize, retained: &mut usize) -> PdfObject {
    let mut grant = CosGrant { work: 1, ..Default::default() };
    for _ in 0..1024 {
        if let ObjectStreamEvent::Complete { end: actual } = controlled_step(cursor, source, &mut grant, retained, false).unwrap() {
            assert_eq!(actual, end);
            let value = cursor.take_value().unwrap();
            assert!(cursor.take_value().is_none());
            for _ in 0..128 {
                if controlled_step(cursor, source, &mut grant, retained, false) == Ok(ObjectStreamEvent::Idle) { return value; }
            }
            panic!("member scratch did not retire");
        }
    }
    panic!("selected object did not finish");
}

#[test]
fn indexed_stream_selects_exact_fixture_members_and_transfers_only_typed_capacity() {
    let decoded = decoded_fixture();
    let mut cursor = ObjectStreamCursor::new(2, 26, CosLimits::default());
    let mut retained = 0;
    assert!(cursor.select(6, 2).is_err());
    cursor.select(6, 0).unwrap();
    assert!(cursor.select(7, 1).is_err());
    let dictionary = selected_value(&mut cursor, &decoded, 70, &mut retained);
    assert_eq!(dictionary.dict_get("Title").unwrap().as_str_bytes(), Some(b"Compressed".as_slice()));
    assert_eq!(dictionary.dict_get("Subject").unwrap().as_str_bytes(), Some(b"Stream".as_slice()));
    assert!(cursor.select(6, 0).is_err());
    cursor.select(7, 1).unwrap();
    let array = selected_value(&mut cursor, &decoded, 102, &mut retained);
    assert_eq!(array.as_array().unwrap(), [
        PdfObject::Bool(true), PdfObject::Name("Object".into()), PdfObject::Str(b"packed".to_vec()),
        PdfObject::Ref(ObjRef { num: 6, gen: 0 }),
    ]);
    assert!(retained > 0);
    for value in [dictionary, array] {
        let mut cursor = PdfCosCursor::retire_parsed_value(value);
        let mut grant = CosGrant { work: 1, ..Default::default() };
        for _ in 0..256 {
            let step = cursor.close_step(grant);
            assert!(step.work <= grant.work && step.released_bytes <= grant.release_bytes);
            retained -= step.released_bytes;
            match step.status.unwrap() {
                CosStatus::Closed => break,
                CosStatus::NeedsRelease { bytes } => grant.release_bytes = bytes,
                _ => grant.release_bytes = 0,
            }
        }
        assert!(cursor.terminal_is_empty());
    }
    assert_eq!(retained, 0);
}

#[test]
fn indexed_stream_rejects_wrong_identity_and_trailing_values_without_lending() {
    let source = b"6 0 7 8 true 42 false";
    let mut cursor = ObjectStreamCursor::new(2, 8, CosLimits::default());
    let grant = CosGrant { work: 32, ..Default::default() };
    cursor.select(7, 0).unwrap();
    loop {
        match cursor.advance(source, grant, false).event {
            Ok(ObjectStreamEvent::More) => {}
            result => { assert_eq!(result, Err("object stream member number differs from cross-reference")); break; }
        }
    }
    cursor.select(6, 0).unwrap();
    loop {
        match cursor.advance(source, grant, false).event {
            Ok(ObjectStreamEvent::More) => {}
            result => { assert_eq!(result, Err("unexpected bytes after object stream member")); break; }
        }
    }
    assert!(cursor.take_value().is_none());
}

#[test]
fn indexed_stream_skips_unselected_members_and_retires_on_cancellation() {
    let source = b"6 0 7 2 ? [(long string) /name]";
    let mut cursor = ObjectStreamCursor::new(2, 8, CosLimits::default());
    cursor.select(7, 1).unwrap();
    let mut grant = CosGrant { work: 1, ..Default::default() };
    let mut retained = 0;
    for _ in 0..256 {
        assert!(controlled_step(&mut cursor, source, &mut grant, &mut retained, false).is_ok());
        if retained != 0 { break; }
    }
    assert!(retained > 0);
    let stopped = cursor.advance(source, grant, true);
    assert_eq!(stopped.event, Ok(ObjectStreamEvent::Cancelled));
    assert_eq!(stopped.work + stopped.allocated_bytes + stopped.released_bytes, 0);
    assert!(cursor.take_value().is_none());
    cursor.begin_close();
    for _ in 0..256 {
        let step = cursor.close_step(grant);
        assert!(step.work <= grant.work && step.allocated_bytes == 0 && step.released_bytes <= grant.release_bytes);
        retained -= step.released_bytes;
        match step.event.unwrap() {
            ObjectStreamEvent::Closed => { assert_eq!(retained, 0); return; }
            ObjectStreamEvent::NeedsRelease { bytes } => grant.release_bytes = bytes,
            _ => grant.release_bytes = 0,
        }
    }
    panic!("cancelled member did not retire");
}
