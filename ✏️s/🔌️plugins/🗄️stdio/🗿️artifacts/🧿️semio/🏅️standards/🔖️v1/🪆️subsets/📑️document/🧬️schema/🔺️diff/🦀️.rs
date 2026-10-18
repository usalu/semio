//! 🔺️ SemioDocumentDiff — handcrafted sparse diff over `SemioDocumentSnapshot`
//! (`styles`/`images`/`blocks`). No `snapshot: Option<SemioDocumentSnapshot>` full-replace slot —
//! even `SetSnapshot`'s diff is the sparse field-by-field `SemioDocumentDiff::between(base, next)`.
//!
//! `blocks` is a recursive tree (`List`/`Table`/`Quote` all nest `DocBlock` further down — list
//! items, table cells, blockquote bodies), diffed with the same index-keyed recursive-triple
//! pattern xml/svg/md/docx use, generalized here via the STANDARD-LEVEL shared
//! `engine::triples::{IndexedTripleDiff, NamedTripleDiff, NamedAdded}` (per this ticket's brief: reuse the
//! real, tested generic collection-diff engine instead of re-deriving docx's own local copy).
//! `styles`/`images` are name-keyed (by `DocStyle::id`/`DocImage::id`) via the same module's
//! `NamedTripleDiff`. The generic BETWEEN/APPLY/INVERSE/ABSORB engine functions themselves
//! (`between_indexed`/`apply_indexed`/`inverse_indexed`/`absorb_indexed` and their named-triple
//! twins) are not part of `engine::triples` (that module only owns the wire TYPES + text codec) —
//! per-artifact local copies are the established convention (docx keeps its own copy too).
//!
//! 🧪️ Per f6-final-summary.md §4.4/§4.3: `#[derive(dsl::DslDiff)]` would fail here for the exact
//! same two reasons docx hit — `DocBlockDiff` is a genuine data-carrying enum reachable through
//! `IndexedTripleDiff<DocBlockDiff, DocBlock>` (`DslField` has no impl for either the enum or the
//! generic collection-triple wrapping it), and every tri-state field below
//! (`style_id`/`based_on`/`width`/`height`/`size`/`font`/`color`/`link`: all `Option<Option<T>>`)
//! fails with `Option<T>: DslField` is not satisfied. `DiffBinary,DiffCodec,DiffText` is hand-rolled below, following
//! the svg/gif/docx template exactly.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexAdded;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedAdded;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedModified;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;

use crate::schema::snapshot::{DocBlock, DocImage, DocListItem, DocRun, DocStyle, DocTableCell, DocTableRow, RunStyle, SemioDocumentSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

#[path = "🧬️diff-types/🦀️.rs"]
mod diff_types;
pub use diff_types::*;

#[path = "⚙️indexed-and-named-engine/🦀️.rs"]
mod indexed_and_named_engine;
use indexed_and_named_engine::*;

//#region 🔖️DocumentDiffLogic

#[path = "🖊️diff-logic-and-algebra/🦀️.rs"]
mod diff_logic_and_algebra;

/// 🌱 Base fixture for `between_demo_cases`/`diff_codec_text_binary_roundtrip_law` — module-scope
/// (not `mod tests`-local) so both this facet's own tests and `between_demo_cases` share one source of
/// truth (model/flow's own `sweep_a`/`sweep_b` promotion precedent).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_a() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: "s.stdio.semio.document".into(),
        styles: vec![DocStyle { id: "keep".into(), name: "Keep".into(), based_on: Some("toRemove".into()) }, DocStyle { id: "toRemove".into(), name: "Gone".into(), based_on: None }],
        images: vec![DocImage { id: "toRemove".into(), mime: "image/png".into(), bytes: vec![9, 9] }],
        blocks: vec![
            DocBlock::Paragraph { style_id: None, runs: vec![DocRun { text: "old".into(), style: RunStyle { bold: false, ..Default::default() } }] },
            DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] },
        ],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_b() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: "s.stdio.semio.document".into(),
        styles: vec![DocStyle { id: "keep".into(), name: "Keep2".into(), based_on: None }, DocStyle { id: "added".into(), name: "Added".into(), based_on: None }],
        images: vec![DocImage { id: "added".into(), mime: "image/jpeg".into(), bytes: vec![1] }],
        blocks: vec![DocBlock::Paragraph { style_id: Some("keep".into()), runs: vec![DocRun { text: "new".into(), style: RunStyle { bold: true, italic: true, ..Default::default() } }, DocRun::plain("second")] }],
    }
}
//#region 🔖️Demo
/// 🌱 Representative `SemioDocumentDiff` cases built declaratively (empty/no-op and an empty-but-present row triple per collection) — single source of truth for `diff_grammar_conformance_law`/`protocol_walk_law` in
/// `🎹️composer/🦀️.rs`.
#[cfg(any(test, feature = "testkit"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_diff_cases() -> Vec<SemioDocumentDiff> {
    vec![SemioDocumentDiff::default(), SemioDocumentDiff { styles: Some(Default::default()), images: Some(Default::default()), blocks: Some(Default::default()) }]
}
//#endregion 🔖️Demo

//#endregion 🔖️DocumentDiffLogic
