//! 🧫️ Typed COS assembly and custody from a genuine PDF 1.7 object.

use super::*;
use crate::PdfDecimal;

const SOURCE: &[u8] = include_bytes!("🧪️containers.pdf");

fn name(value: &str) -> PdfObject { PdfObject::Name(value.into()) }
fn entry(key: &str, value: PdfObject) -> PdfDictEntry { PdfDictEntry { key: key.into(), value } }

fn expected() -> PdfObject {
    use PdfObject::{Array, Bool, Dict, Int, Null, Real, Ref, Str};
    Dict(vec![
        entry("Numbers", Array(vec![Int(12), Int(0), Ref(ObjRef { num: 6, gen: 0 }), Int(-17),
            Real(PdfDecimal { negative: false, coefficient: "050".into(), scale: 2 }),
            Real(PdfDecimal { negative: false, coefficient: "9".into(), scale: 0 }), Int(602214076)])),
        entry("Text", Str(b"a(b)+".to_vec())),
        entry("Hex", Str(vec![0xa4, 0xf0])),
        entry("Flags", Array(vec![Bool(true), Bool(false), Null])),
        entry("Nested", Dict(vec![
            entry("A B", Array(vec![name("x"), name("é"), name("é"), Dict(vec![entry("Empty", Array(Vec::new()))])])),
            entry("First", name("first")),
            entry("Second", name("second")),
        ])),
    ])
}

fn offset() -> usize {
    let header = b"4 0 obj\n";
    SOURCE.windows(header.len()).position(|bytes| bytes == header).unwrap() + header.len()
}

fn next_grant(status: CosStatus) -> CosGrant {
    CosGrant {
        work: 1,
        allocation_bytes: if let CosStatus::NeedsAllocation { bytes } = status { bytes } else { 0 },
        release_bytes: if let CosStatus::NeedsRelease { bytes } = status { bytes } else { 0 },
    }
}

fn limits(receipt: &CosReceipt, grant: CosGrant) {
    assert!(receipt.work <= grant.work);
    assert!(receipt.allocated_bytes <= grant.allocation_bytes);
    assert!(receipt.released_bytes <= grant.release_bytes);
}

fn drive(cursor: &mut PdfCosCursor, data: &[u8], stop: impl Fn(&PdfCosCursor) -> bool) -> (Result<CosStatus, &'static str>, usize) {
    let mut next = next_grant(CosStatus::More);
    let mut retained = 0;
    for _ in 0..20_000 {
        let before = cursor.inspected_position();
        let receipt = cursor.advance(data, next, false);
        limits(&receipt, next);
        retained += receipt.allocated_bytes;
        retained -= receipt.released_bytes;
        let end = cursor.inspected_position();
        assert!(end >= before && end - before <= 1);
        match receipt.status {
            Ok(status @ (CosStatus::More | CosStatus::NeedsAllocation { .. } | CosStatus::NeedsRelease { .. })) => {
                assert!(cursor.take_value().is_none());
                if stop(cursor) {
                    let cancelled = cursor.advance(data, next, true);
                    assert_eq!(cancelled, CosReceipt { status: Ok(CosStatus::Cancelled), work: 0, allocated_bytes: 0, released_bytes: 0 });
                    assert_eq!(cursor.inspected_position(), end);
                    return (cancelled.status, retained);
                }
                next = next_grant(status);
            }
            status => return (status, retained),
        }
    }
    panic!("COS cursor did not complete its bounded drive");
}

fn close(cursor: &mut PdfCosCursor, retained: usize) {
    cursor.begin_close();
    assert_eq!(cursor.close_step(CosGrant::default()).work, 0);
    let mut grant = next_grant(CosStatus::More);
    let mut released = 0;
    for _ in 0..20_000 {
        let closed = cursor.close_step(grant);
        limits(&closed, grant);
        released += closed.released_bytes;
        let status = closed.status.unwrap();
        if status == CosStatus::Closed {
            assert_eq!(released, retained);
            assert!(cursor.terminal_is_empty());
            return;
        }
        grant = next_grant(status);
    }
    panic!("COS cursor retained capacity after retirement drive");
}

#[test]
fn pdf_cos_assembles_native_types_at_single_byte_grants() {
    let mut cursor = PdfCosCursor::new(offset(), CosLimits::default());
    let (status, _) = drive(&mut cursor, SOURCE, |_| false);
    let CosStatus::Complete { end } = status.unwrap() else { panic!("expected COS completion") };
    assert!(SOURCE[end..].starts_with(b"\nendobj"));
    assert_eq!(cursor.take_value(), Some(expected()));
    let (reference, production) = crate::parse_indirect_at(SOURCE, offset() - b"4 0 obj\n".len()).unwrap();
    assert_eq!(reference, ObjRef { num: 4, gen: 0 });
    assert_eq!(production, expected());
    assert!(cursor.take_value().is_none());
    close(&mut cursor, 0);
}

#[test]
fn pdf_cos_retires_completed_partial_and_invalid_ownership() {
    let mut cursor = PdfCosCursor::new(offset(), CosLimits::default());
    let (status, retained) = drive(&mut cursor, SOURCE, |_| false);
    assert!(matches!(status, Ok(CosStatus::Complete { .. })) && retained > 0);
    close(&mut cursor, retained);

    let mut cursor = PdfCosCursor::new(offset(), CosLimits::default());
    let (status, retained) = drive(&mut cursor, SOURCE, |cursor| cursor.inspected_position() > offset() + 100);
    assert_eq!(status, Ok(CosStatus::Cancelled));
    assert!(retained > 0);
    close(&mut cursor, retained);

    for input in [b"<< /Name [1 2 3] /Missing >>".as_slice(), b"[<< /X (unfinished", b"[ /Name >>", b"<< [1] /X >>"] {
        let mut cursor = PdfCosCursor::new(0, CosLimits::default());
        let (status, retained) = drive(&mut cursor, input, |_| false);
        assert!(status.is_err());
        close(&mut cursor, retained);
    }
}

#[test]
fn pdf_cos_keeps_exact_decimal_and_scalar_boundary() {
    let data = b"9223372036854775808  /follow";
    let mut cursor = PdfCosCursor::new(0, CosLimits::default());
    let (status, _) = drive(&mut cursor, data, |_| false);
    assert_eq!(status, Ok(CosStatus::Complete { end: 19 }));
    assert_eq!(cursor.take_value(), Some(PdfObject::Real(PdfDecimal {
        negative: false, coefficient: "9223372036854775808".into(), scale: 0,
    })));
    close(&mut cursor, 0);

    let mut cursor = PdfCosCursor::new(0, CosLimits::default());
    let (status, _) = drive(&mut cursor, b"42  % comment\n /follow", |_| false);
    assert_eq!(status, Ok(CosStatus::Complete { end: 2 }));
    assert_eq!(cursor.take_value(), Some(PdfObject::Int(42)));
    close(&mut cursor, 0);
}

#[test]
fn pdf_cos_retires_during_vector_migration_and_enforces_limits() {
    let data = b"[1 2 3 4 5]";
    let mut cursor = PdfCosCursor::new(0, CosLimits::default());
    let mut retained = 0;
    let mut grant = next_grant(CosStatus::More);
    for _ in 0..100 {
        let receipt = cursor.advance(data, grant, false);
        limits(&receipt, grant);
        let newly_allocated = receipt.allocated_bytes != 0 && retained != 0;
        retained += receipt.allocated_bytes - receipt.released_bytes;
        if newly_allocated {
            assert_eq!(cursor.advance(data, grant, true).status, Ok(CosStatus::Cancelled));
            close(&mut cursor, retained);
            break;
        }
        grant = next_grant(receipt.status.unwrap());
    }
    assert!(cursor.terminal_is_empty());

    let cases = [
        (b"[[]]".as_slice(), CosLimits { depth: 1, ..CosLimits::default() }, "PDF COS nesting limit exceeded"),
        (b"[1 2 3]", CosLimits { container_items: 2, ..CosLimits::default() }, "PDF COS item limit exceeded"),
        (b"(abc)", CosLimits { string_bytes: 2, ..CosLimits::default() }, "PDF COS item limit exceeded"),
        (b"/long", CosLimits { scalar_bytes: 2, ..CosLimits::default() }, "PDF COS scalar token limit exceeded"),
    ];
    for (data, limit, expected) in cases {
        let mut cursor = PdfCosCursor::new(0, limit);
        let (status, retained) = drive(&mut cursor, data, |_| false);
        assert_eq!(status, Err(expected));
        close(&mut cursor, retained);
    }
}
