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

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};


use crate::standards::v1::subsets::document::schema::diff::{BlocksDiff, DocBlockDiff, DocHeadingDiff, DocImageBlockDiff, DocParagraphDiff, DocQuoteDiff, DocRunDiff, DocTableCellDiff, DocTableRowDiff, ListItemsDiff, RunsDiff, SemioDocumentDiff, TableCellsDiff, TableRowsDiff};


















use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocRun, DocStyle, RunStyle, SemioDocumentSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioDocumentMutation` block
/// below calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in
/// scope in production code too, not merely under `#[cfg(test)]` (W2b closer fix).


//#region 🔖️PathAddressing
/// 🧭️ One step down into a nested block container: `Quote` (own `blocks`), a `List` item's own
/// `blocks`, or a `Table` cell's own `blocks`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum DocPathSegment {
    Quote { block_index: usize },
    ListItem { block_index: usize, item: usize },
    TableCell { block_index: usize, row: usize, cell: usize },
}

/// 🧭️ Addresses one block-list slot: `segments` navigate through nested containers, `index` is
/// the slot within the innermost `Vec<DocBlock>`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DocBlockPath {
    #[value(default)]
    pub segments: Vec<DocPathSegment>,
    pub index: usize,
}

impl DocBlockPath {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn top(index: usize) -> Self {
        Self { segments: Vec::new(), index }
    }
}

/// 🧭️ Resolves the block list a path's segments navigate to (the parent list `path.index` slots
/// into), immutable form. `pub(crate)` so builder/composer callers can reuse it without
/// duplicating the traversal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn resolve_blocks<'a>(body: &'a [DocBlock], segments: &[DocPathSegment]) -> Option<&'a [DocBlock]> {
    match segments.split_first() {
        None => Some(body),
        Some((seg, rest)) => match seg {
            DocPathSegment::Quote { block_index } => {
                let DocBlock::Quote { blocks } = body.get(*block_index)? else { return None };
                resolve_blocks(blocks, rest)
            }
            DocPathSegment::ListItem { block_index, item } => {
                let DocBlock::List { items, .. } = body.get(*block_index)? else { return None };
                resolve_blocks(&items.get(*item)?.blocks, rest)
            }
            DocPathSegment::TableCell { block_index, row, cell } => {
                let DocBlock::Table { rows } = body.get(*block_index)? else { return None };
                resolve_blocks(&rows.get(*row)?.cells.get(*cell)?.blocks, rest)
            }
        },
    }
}

enum DocBlockLeaf {
    Modified(DocBlockDiff),
    Inserted(DocBlock),
    Removed,
}

impl DocBlockLeaf {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn into_blocks_diff(self, index: usize) -> BlocksDiff {
        match self {
            Self::Modified(diff) => BlocksDiff { modified: vec![IndexModified { index, diff }], ..Default::default() },
            Self::Inserted(block) => BlocksDiff { added: vec![IndexAdded { index, item: block }], ..Default::default() },
            Self::Removed => BlocksDiff { removed: vec![index], ..Default::default() },
        }
    }
}

/// 🧭️ Lowers a `leaf` diff targeting the block addressed by `path` into a full
/// `SemioDocumentDiff` by nesting it through `Quote`/`List`/`Table` from the document root down
/// to that depth (mirrors docx's `wrap_body_diff`, generalized to 3 container kinds).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_body_diff(path: &DocBlockPath, leaf: DocBlockLeaf) -> SemioDocumentDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn go(segments: &[DocPathSegment], index: usize, leaf: DocBlockLeaf) -> BlocksDiff {
        match segments.split_first() {
            None => leaf.into_blocks_diff(index),
            Some((seg, rest)) => {
                let inner = go(rest, index, leaf);
                match seg {
                    DocPathSegment::Quote { block_index } => {
                        let qd = DocBlockDiff::Quote(DocQuoteDiff { blocks: Some(inner) });
                        BlocksDiff { modified: vec![IndexModified { index: *block_index, diff: qd }], ..Default::default() }
                    }
                    DocPathSegment::ListItem { block_index, item } => {
                        let item_diff = crate::standards::v1::subsets::document::schema::diff::DocListItemDiff { blocks: Some(inner) };
                        let items_diff: ListItemsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *item, diff: item_diff }], ..Default::default() };
                        let ld = DocBlockDiff::List(crate::standards::v1::subsets::document::schema::diff::DocListDiff { ordered: None, items: Some(items_diff) });
                        BlocksDiff { modified: vec![IndexModified { index: *block_index, diff: ld }], ..Default::default() }
                    }
                    DocPathSegment::TableCell { block_index, row, cell } => {
                        let cell_diff = DocTableCellDiff { blocks: Some(inner) };
                        let cells_diff: TableCellsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *cell, diff: cell_diff }], ..Default::default() };
                        let row_diff = DocTableRowDiff { cells: Some(cells_diff) };
                        let rows_diff: TableRowsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *row, diff: row_diff }], ..Default::default() };
                        let td = DocBlockDiff::Table(crate::standards::v1::subsets::document::schema::diff::DocTableDiff { rows: Some(rows_diff) });
                        BlocksDiff { modified: vec![IndexModified { index: *block_index, diff: td }], ..Default::default() }
                    }
                }
            }
        }
    }
    let blocks = go(&path.segments, path.index, leaf);
    SemioDocumentDiff { styles: None, images: None, blocks: Some(blocks) }
}
//#endregion 🔖️PathAddressing

//#region 🔖️Mutations
//#region 🔖️Leaves
#[path = "🧱insert-block/🦀️.rs"]
pub mod insert_block;
#[path = "🖼️insert-image/🦀️.rs"]
pub mod insert_image;
#[path = "🧶insert-style/🦀️.rs"]
pub mod insert_style;
#[path = "🪓remove-block/🦀️.rs"]
pub mod remove_block;
#[path = "🪦remove-image/🦀️.rs"]
pub mod remove_image;
#[path = "🧽️remove-style/🦀️.rs"]
pub mod remove_style;
#[path = "📦set-block-content/🦀️.rs"]
pub mod set_block_content;
#[path = "📐set-heading-level/🦀️.rs"]
pub mod set_heading_level;
#[path = "📷set-image-block/🦀️.rs"]
pub mod set_image_block;
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = SemioDocumentSnapshot, diff = SemioDocumentDiff, schema = "SemioDocumentMutation")]
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
    <SemioDocumentMutation as protocol::Mutation<SemioDocumentSnapshot>>::diff(mutation, base)
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
    Mutation::inverse(mutation, base)?

    })
}


//#endregion 🔖️Apply

//#region 🔖️Helpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn block_at<'a>(base: &'a SemioDocumentSnapshot, path: &DocBlockPath) -> Option<&'a DocBlock> {
    resolve_blocks(&base.blocks, &path.segments)?.get(path.index)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn style_at<'a>(base: &'a SemioDocumentSnapshot, id: &str) -> Option<&'a DocStyle> {
    base.styles.iter().find(|s| s.id == id)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn image_at<'a>(base: &'a SemioDocumentSnapshot, id: &str) -> Option<&'a DocImage> {
    base.images.iter().find(|i| i.id == id)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn runs_of(block: &DocBlock) -> Option<&Vec<DocRun>> {
    match block {
        DocBlock::Paragraph { runs, .. } | DocBlock::Heading { runs, .. } => Some(runs),
        _ => None,
    }
}
/// 🎯️ Wraps a `RunsDiff` into the right `DocBlockDiff` variant depending on whether `block` is a
/// `Paragraph` or a `Heading` (the only two run-carrying kinds).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_runs_diff(block: &DocBlock, runs: RunsDiff) -> Option<DocBlockDiff> {
    match block {
        DocBlock::Paragraph { .. } => Some(DocBlockDiff::Paragraph(DocParagraphDiff { style_id: None, runs: Some(runs) })),
        DocBlock::Heading { .. } => Some(DocBlockDiff::Heading(DocHeadingDiff { level: None, style_id: None, runs: Some(runs) })),
        _ => None,
    }
}
//#endregion 🔖️Helpers




//#endregion 🔖️MutationTrait

//#region OpCodecs


























//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Representative `SemioDocumentMutation` cases (one per variant) — single source of truth for
/// this facet's own `op_text_binary_roundtrip_law` AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioDocumentMutation> {
    let table_block =
        DocBlock::Table { rows: vec![crate::standards::v1::subsets::document::schema::snapshot::DocTableRow { cells: vec![crate::standards::v1::subsets::document::schema::snapshot::DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] };
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
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{OpBinary,OpText};
