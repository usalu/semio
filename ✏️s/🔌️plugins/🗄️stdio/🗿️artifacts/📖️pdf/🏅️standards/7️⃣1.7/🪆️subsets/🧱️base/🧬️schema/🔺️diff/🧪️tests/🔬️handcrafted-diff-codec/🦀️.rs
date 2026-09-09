
use super::*;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfIndirectObject, STDIO_PDF17_DOCUMENT_SCHEMA};
use protocol::DiffCodec;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn oref(num: u32, gen: u16) -> ObjRef {
    ObjRef { num, gen }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn page(mb: [f64; 4], cb: Option<[f64; 4]>, rotate: i32, text: &str) -> PdfPage {
    PdfPage { media_box: mb, crop_box: cb, rotate, text: text.into() }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn dict(entries: Vec<(&str, PdfObject)>) -> PdfObject {
    PdfObject::Dict(entries.into_iter().map(|(k, v)| PdfDictEntry { key: k.into(), value: v }).collect())
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn entry(k: &str, v: PdfObject) -> PdfDictEntry {
    PdfDictEntry { key: k.into(), value: v }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn a_snapshot() -> PdfSnapshot {
    PdfSnapshot {
        schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
        declared_version: "1.7".into(),
        pages: vec![page([0.0, 0.0, 100.0, 100.0], None, 0, "one"), page([0.0, 0.0, 50.0, 50.0], Some([1.0, 1.0, 2.0, 2.0]), 0, "two")],
        info: PdfInfo { title: Some("Base".into()), ..Default::default() },
        objects: vec![
            PdfIndirectObject { id: oref(1, 0), value: dict(vec![("Type", PdfObject::Name("Catalog".into())), ("Count", PdfObject::Int(3))]) },
            PdfIndirectObject { id: oref(2, 0), value: PdfObject::Stream { dict: vec![entry("Length", PdfObject::Int(3))], data: vec![1, 2, 3], filters: vec![PdfStreamFilter::Flate { predictor: None }] } },
            PdfIndirectObject { id: oref(3, 0), value: PdfObject::Array(vec![PdfObject::Int(1), PdfObject::Real(2.5.into()), PdfObject::Ref(oref(1, 0))]) },
        ],
        trailer: vec![entry("Root", PdfObject::Ref(oref(1, 0))), entry("Size", PdfObject::Int(3))],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn b_snapshot() -> PdfSnapshot {
    PdfSnapshot {
        schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
        declared_version: "1.4".into(),
        pages: vec![page([0.0, 0.0, 200.0, 200.0], None, 90, "ONE")],
        info: PdfInfo { title: Some("Changed".into()), author: Some("Ueli".into()), ..Default::default() },
        objects: vec![
            PdfIndirectObject { id: oref(1, 0), value: dict(vec![("Type", PdfObject::Name("Catalog".into())), ("Count", PdfObject::Int(4)), ("New", PdfObject::Bool(false))]) },
            PdfIndirectObject { id: oref(2, 0), value: PdfObject::Stream { dict: vec![entry("Length", PdfObject::Int(3))], data: vec![9, 9], filters: vec![] } },
            PdfIndirectObject { id: oref(4, 0), value: PdfObject::Null },
        ],
        trailer: vec![entry("Root", PdfObject::Ref(oref(1, 0))), entry("Size", PdfObject::Int(4)), entry("Prev", PdfObject::Int(100))],
    }
}

/// 🧪️ F6: `DiffCodec` round-trip laws over the hand-rolled `PdfDiff` grammar — exercises the
/// recursive `PdfValueDiff` tree (`Replace`/`Array`/`Dict`/`Stream` variants, incl. `Stream`'s
/// own typed filter pipeline), the index-keyed `pages` triple (incl. `PdfPageDiff`'s tri-state
/// `crop_box`), the id-keyed `objects` triple, and the name-keyed `trailer` triple.
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    let a = a_snapshot();
    let b = b_snapshot();
    let cases = vec![PdfDiff::default(), PdfDiff::between(&a, &b), PdfDiff::between(&b, &a), PdfDiff::between(&a, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = PdfDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = PdfDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
#[test]
fn rejects_missing_page_target_without_mutating_base() {
    let base = PdfSnapshot::default();
    let diff = PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff::default() }], ..Default::default() }), ..Default::default() };
    let result = diff.apply(&base);
    assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
    assert_eq!(base, PdfSnapshot::default());
}
