//! 🧪️ Fixtures and appliers of the PDF 1.7 conformance mutation tests: documents built from objects or read back from their own encoding, the
//! committed `lopdf` pairs, edit extractors, and the central applier. A regular module of the schema so a test of any crate can name it without
//! reaching into another crate's `cfg(test)` items.

use crate::standards::v1_7::subsets::base::io::export::encode_pdf;
use crate::standards::v1_7::subsets::base::io::import::decode_pdf;
use crate::standards::v1_7::subsets::base::schema::conformance_support::*;
use crate::standards::v1_7::subsets::base::schema::diff::{graph_edit, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfIndirectObject, PdfObject, PdfPage, PdfSnapshot};

/// 🧪️ A document holding exactly `values` as the indirect objects `1 0 R`, `2 0 R`, … in order, with no typed lane and no trailer.
pub fn document_of(values: Vec<PdfObject>) -> PdfSnapshot {
    PdfSnapshot { objects: values.into_iter().enumerate().map(|(index, value)| PdfIndirectObject { id: ObjRef { num: index as u32 + 1, gen: 0 }, value }).collect(), ..PdfSnapshot::default() }
}

/// 🧪️ A real one-page document read back from its own encoding, so its typed lanes and its retained graph agree.
pub fn document() -> PdfSnapshot {
    let typed = PdfSnapshot { pages: vec![PdfPage::new(612.0, 792.0)], ..PdfSnapshot::default() };
    decode_pdf(&encode_pdf(&typed).expect("the fixture document encodes")).expect("the fixture document decodes")
}

/// 🧪️ A committed `lopdf` pair read through this subset's reader: `(before, after)`.
pub fn snapshots(before: &[u8], after: &[u8]) -> (PdfSnapshot, PdfSnapshot) {
    (decode_pdf(before).expect("the committed before-document decodes"), decode_pdf(after).expect("the committed after-document decodes"))
}

/// 🧪️ The item `after` holds and `before` lacks.
pub fn added<T: Clone + PartialEq>(before: &[T], after: &[T]) -> T {
    after.iter().find(|item| !before.contains(item)).cloned().expect("the committed edit adds exactly this item")
}

/// 🧪️ The item `before` holds and `after` lacks.
pub fn removed<T: Clone + PartialEq>(before: &[T], after: &[T]) -> T {
    before.iter().find(|item| !after.contains(item)).cloned().expect("the committed edit removes exactly this item")
}

/// 🧪️ The first page's operator edit as `(at, removed, inserted)`: the common prefix, the operators dropped after it, and the
/// operators written in their place.
pub fn content_edit(before: &PdfSnapshot, after: &PdfSnapshot) -> (usize, usize, Vec<crate::standards::v1_7::subsets::base::schema::snapshot::PdfOp>) {
    let (old, new) = (&before.pages[0].content, &after.pages[0].content);
    let at = old.iter().zip(new).take_while(|(left, right)| left == right).count();
    let tail = old[at..].iter().rev().zip(new[at..].iter().rev()).take_while(|(left, right)| left == right).count();
    (at, old.len() - at - tail, new[at..new.len() - tail].to_vec())
}

/// 🧪️ The smallest document a catalog entry can hang off: one `/Type /Catalog` root.
pub fn catalog_object() -> PdfObject {
    dict(vec![("Type", PdfObject::Name("Catalog".to_string()))])
}

/// 🧪️ `base` with one `/Type /Tail` object appended and, when the document has a catalog, one `/Tail` catalog entry appended —
/// the rows that make everything created before them a MIDDLE row.
pub fn with_tail(base: &PdfSnapshot) -> PdfSnapshot {
    let (_, rows) = insert_object_rows(base, dict(vec![("Type", PdfObject::Name("Tail".to_string()))]), &[]);
    let base = after_rows(base, rows);
    match catalog_id(&base) {
        Some(_) => after_rows(&base, set_catalog_entry_rows(&base, "Tail", PdfObject::Int(1), None)),
        None => base,
    }
}

/// 🧪️ `base` with a `/Tail` entry appended to object `id`'s own dictionary, so every entry before it becomes a MIDDLE entry.
pub fn with_trailing_entry(base: &PdfSnapshot, id: ObjRef) -> PdfSnapshot {
    after_rows(base, set_entry_rows(base, id, "Tail", PdfObject::Int(1), None))
}

/// 🧪️ The placement of object number `num` at list position `index`.
pub fn placed(num: u32, index: usize) -> ObjectPlacement {
    ObjectPlacement { id: ObjRef { num, gen: 0 }, index }
}

/// 🧪️ `base` after the `rows` of one edit, applied through the central applier as a graph edit.
pub fn after_rows(base: &PdfSnapshot, rows: PdfDiff) -> PdfSnapshot {
    protocol::apply_diff_cold(&graph_edit(rows), base).expect("the fixture rows apply")
}

/// 🧪️ `base` after `mutation`, applied through the central applier.
pub fn applied<M: protocol::Mutation<PdfSnapshot, Diff = PdfDiff>>(base: &PdfSnapshot, mutation: &M) -> PdfSnapshot {
    protocol::apply_diff_cold(mutation.diff(base).diff(), base).expect("the fixture mutation applies")
}
