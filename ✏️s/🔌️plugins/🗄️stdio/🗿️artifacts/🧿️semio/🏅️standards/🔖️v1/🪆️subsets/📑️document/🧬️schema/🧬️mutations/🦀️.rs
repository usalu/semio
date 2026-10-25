//! 🧬️ SemioDocumentMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (never apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.
//! Addresses the recursive `blocks` tree via `DocBlockPath` (segments navigate through nested
//! `Quote`/`List`/`Table` containers — svg's `NodePath` precedent, extended for 3 nesting kinds
//! instead of docx's single `Table`-only nesting), named styles by `DocStyle::id`, and named
//! images by `DocImage::id`.
//!
//! 🧪️ Per f6-final-summary.md §4.3/§4.4: `#[derive(dsl::DslOps)]` would fail here for the same
//! reasons `DocxMutation` hit — `SetSnapshot{snapshot}` reaches the data-carrying `DocBlockDiff`
//! enum transitively; `InsertBlock`/`SetBlockContent`'s bare `block: DocBlock` fails directly
//! (`DocBlock: DslField` not satisfied — it's a data-carrying enum); `style: RunStyle`/
//! `path: DocBlockPath` also fail (`DslField` not satisfied, neither is `#[derive(DslRecord)]`).
//! `OpText`/`OpBinary` hand-rolled below, reusing `SemioDocumentDiff`'s `pub(crate)` grammar
//! primitives.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexAdded;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;


use crate::schema::diff::{BlocksDiff, DocBlockDiff, DocHeadingDiff, DocImageBlockDiff, DocParagraphDiff, DocQuoteDiff, DocRunDiff, DocTableCellDiff, DocTableRowDiff, ListItemsDiff, RunsDiff, SemioDocumentDiff, TableCellsDiff, TableRowsDiff};


















use crate::schema::snapshot::{DocBlock, DocImage, DocRun, DocStyle, RunStyle, SemioDocumentSnapshot};
use protocol::{ColdMutation, Mutation};
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioDocumentMutation` block
/// below calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in
/// scope in production code too, not merely under `#[cfg(test)]` (W2b closer fix).



//#region 🔖️Mutations
//#region 🔖️Leaves
pub use semio_s_artifact_stdio_semio_document_examples::schema::mutations::support;
pub use support::{DocBlockPath, DocPathSegment};
pub(crate) use support::{resolve_blocks};
#[allow(unused_imports)]
use support::{DocBlockLeaf, block_at, image_at, runs_of, style_at, wrap_body_diff, wrap_runs_diff};
pub use semio_s_artifact_stdio_semio_document_mut_family::schema::mutations::insert_block;
pub use semio_s_artifact_stdio_semio_document_mut_family::schema::mutations::insert_image;
pub use semio_s_artifact_stdio_semio_document_mut_family::schema::mutations::insert_style;
pub use semio_s_artifact_stdio_semio_document_mut_family::schema::mutations::remove_block;
pub use semio_s_artifact_stdio_semio_document_mut_family::schema::mutations::remove_image;
pub use semio_s_artifact_stdio_semio_document_mut_family::schema::mutations::remove_style;
pub use semio_s_artifact_stdio_semio_document_mut_family::schema::mutations::set_block_content;
pub use semio_s_artifact_stdio_semio_document_mut_family::schema::mutations::set_heading_level;
pub use semio_s_artifact_stdio_semio_document_mut_family::schema::mutations::set_image_block;
#[path = "📀️set-image-bytes/🦀️.rs"]
pub mod set_image_bytes;
#[path = "🔢set-list-ordered/🦀️.rs"]
pub mod set_list_ordered;
#[path = "🪶set-paragraph-style/🦀️.rs"]
pub mod set_paragraph_style;
#[path = "🎨set-run-style/🦀️.rs"]
pub mod set_run_style;
#[path = "🧵set-run-text/🦀️.rs"]
pub mod set_run_text;
#[path = "🧬️set-style-based-on/🦀️.rs"]
pub mod set_style_based_on;
#[path = "🏷️set-style-name/🦀️.rs"]
pub mod set_style_name;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = SemioDocumentSnapshot, diff = SemioDocumentDiff, schema = "SemioDocumentMutation", reference = SemioDocumentSnapshot, reference_diff = SemioDocumentDiff)]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioDocumentMutation {
    /// ➕️ Inserts `block` at `path` (`path.index` = insertion index, FINAL state).
    InsertBlock(insert_block::InsertBlock),
    /// ➖️ Removes the block at `path` (`path.index` = BASE-state index).
    RemoveBlock(remove_block::RemoveBlock),
    /// ✍️ Replaces the full content of the block at `path` with `block` (may change kind).
    SetBlockContent(set_block_content::SetBlockContent),
    /// 🎨️ Sets (or clears) a `Paragraph` block's `style_id`.
    SetParagraphStyle(set_paragraph_style::SetParagraphStyle),
    /// 🔢️ Sets a `Heading` block's `level`.
    SetHeadingLevel(set_heading_level::SetHeadingLevel),
    /// 🔀️ Sets a `List` block's `ordered` flag.
    SetListOrdered(set_list_ordered::SetListOrdered),
    /// ✍️ Replaces the literal text of run `run_index` in the Paragraph/Heading at `path`.
    SetRunText(set_run_text::SetRunText),
    /// 🎨️ Replaces run `run_index`'s full `RunStyle` in the Paragraph/Heading at `path`.
    SetRunStyle(set_run_style::SetRunStyle),
    /// 🖼️ Replaces an `Image` block's `image_id`/`alt`/`width`/`height` at `path`.
    SetImageBlock(set_image_block::SetImageBlock),
    /// ➕️ Inserts a named style.
    InsertStyle(insert_style::InsertStyle),
    /// ➖️ Removes the style with id `id`.
    RemoveStyle(remove_style::RemoveStyle),
    /// 🏷️ Renames the style with id `id`.
    SetStyleName(set_style_name::SetStyleName),
    /// 🔗 Sets (or, if `None`, clears) the style with id `id`'s `based_on`.
    SetStyleBasedOn(set_style_based_on::SetStyleBasedOn),
    /// ➕️ Inserts a named image.
    InsertImage(insert_image::InsertImage),
    /// ➖️ Removes the image with id `id`.
    RemoveImage(remove_image::RemoveImage),
    /// ✍️ Replaces the mime/bytes of the image with id `id`.
    SetImageBytes(set_image_bytes::SetImageBytes),
}

/// 🏷️ The declared mutation vocabulary of `s.stdio.semio.document`, in `SemioDocumentMutation`'s own
/// declaration order and kebab-case spelling — the single source of truth for the binary op frame's
/// `tag` ordinal (see [`wire_tag`]), for `parse_document_mutation`'s keyword match, and for
/// the `semio-v1-document` catalog in `../../🔣️oracle.json`. The framework never parses
/// Rust, so `kinds_match_the_enum_and_the_catalog` below is what keeps all three honest.
pub const KINDS: &[&str] = &[
    "insert-block",
    "remove-block",
    "set-block-content",
    "set-paragraph-style",
    "set-heading-level",
    "set-list-ordered",
    "set-run-text",
    "set-run-style",
    "set-image-block",
    "insert-style",
    "remove-style",
    "set-style-name",
    "set-style-based-on",
    "insert-image",
    "remove-image",
    "set-image-bytes", ];
//#endregion 🔖️Mutations

/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_document_mutation(mutation: &SemioDocumentMutation, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    <SemioDocumentMutation as ColdMutation<SemioDocumentSnapshot>>::diff(mutation, base)
}


/// ↩️ Computes `mutation`'s own inverse against `base` — a thin wrapper around
/// `protocol::Mutation::inverse` so external Rust callers that cannot name this crate's private
/// `protocol` extern-crate item (the `📃️mutate-semio-document` test adapter, whose `inverse-<kind>`
/// scenarios need a mutation's own computed inverse) can still reach the inverse law that
/// `diff_semio_*_mutation` alone cannot. Same shape as `🧰️kit`'s
/// `inverse_semio_kit_mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_document_mutation(mutation: &SemioDocumentMutation, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    Ok({
    ::semio_framework_os_kernel::ColdMutation::inverse(mutation, base)?

    })
}


//#endregion 🔖️Apply





//#endregion 🔖️MutationTrait


//#region 🔖️Demo
/// 🌱 Representative `SemioDocumentMutation` cases (one per variant) — single source of truth for
/// this facet's own `op_text_binary_roundtrip_law` AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️.rs`.
#[cfg(any(test, feature = "testkit"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_mutation_cases() -> Vec<SemioDocumentMutation> {
    let table_block =
        DocBlock::Table { rows: vec![crate::schema::snapshot::DocTableRow { cells: vec![crate::schema::snapshot::DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] };
    vec![
        SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath::top(1), block: table_block.clone() }),
        SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: DocBlockPath { segments: vec![DocPathSegment::TableCell { block_index: 0, row: 0, cell: 0 }], index: 0 }, block: DocBlock::paragraph("nested") }),
        SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path: DocBlockPath::top(0) }),
        SemioDocumentMutation::SetBlockContent(set_block_content::SetBlockContent { path: DocBlockPath::top(0), block: table_block }),
        SemioDocumentMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { path: DocBlockPath::top(0), style_id: None }),
        SemioDocumentMutation::SetHeadingLevel(set_heading_level::SetHeadingLevel { path: DocBlockPath::top(0), level: 2 }),
        SemioDocumentMutation::SetListOrdered(set_list_ordered::SetListOrdered { path: DocBlockPath::top(0), ordered: true }),
        SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path: DocBlockPath::top(0), run_index: 0, text: "hello world".into() }),
        SemioDocumentMutation::SetRunStyle(set_run_style::SetRunStyle { path: DocBlockPath::top(0), run_index: 0, style: RunStyle { bold: true, size: Some(12.0), font: Some("Arial".into()), ..Default::default() } }),
        SemioDocumentMutation::SetImageBlock(set_image_block::SetImageBlock { path: DocBlockPath::top(0), image_id: "img1".into(), alt: "alt".into(), width: Some(10.0), height: None }),
        SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style: DocStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) }, at: None }),
        SemioDocumentMutation::RemoveStyle(remove_style::RemoveStyle { id: "Normal".into() }),
        SemioDocumentMutation::SetStyleName(set_style_name::SetStyleName { id: "Normal".into(), name: "Body Text".into() }),
        SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Other".into()) }),
        SemioDocumentMutation::InsertImage(insert_image::InsertImage { image: DocImage { id: "img2".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] }, at: None }),
        SemioDocumentMutation::RemoveImage(remove_image::RemoveImage { id: "img2".into() }),
        SemioDocumentMutation::SetImageBytes(set_image_bytes::SetImageBytes { id: "img1".into(), mime: "image/gif".into(), bytes: vec![7] }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests

//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{OpBinary,OpText};
