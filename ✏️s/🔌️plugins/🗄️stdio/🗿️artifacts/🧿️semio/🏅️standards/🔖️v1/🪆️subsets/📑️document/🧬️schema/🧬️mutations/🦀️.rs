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


use crate::standards::v1::subsets::document::schema::diff::{diff_block, diff_set_snapshot, BlocksDiff, DocBlockDiff, DocHeadingDiff, DocParagraphDiff, DocQuoteDiff, DocRunDiff, DocTableCellDiff, DocTableRowDiff, ListItemsDiff, RunsDiff, SemioDocumentDiff, TableCellsDiff, TableRowsDiff};


















use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocRun, DocStyle, RunStyle, SemioDocumentSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioDocumentMutation` block
/// below calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in
/// scope in production code too, not merely under `#[cfg(test)]` (W2b closer fix).
use protocol::{OpBinary, OpText};

//#region 🔖️PathAddressing
/// 🧭️ One step down into a nested block container: `Quote` (own `blocks`), a `List` item's own
/// `blocks`, or a `Table` cell's own `blocks`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum DocPathSegment {
    Quote { block_index: usize },
    ListItem { block_index: usize, item: usize },
    TableCell { block_index: usize, row: usize, cell: usize },
}

/// 🧭️ Addresses one block-list slot: `segments` navigate through nested containers, `index` is
/// the slot within the innermost `Vec<DocBlock>`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🧬️set-style-based-on/🦀️.rs"]
pub mod set_style_based_on;
#[path = "🏷️set-style-name/🦀️.rs"]
pub mod set_style_name;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioDocumentSnapshot, diff = SemioDocumentDiff, schema = "SemioDocumentMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioDocumentMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
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
    "set-snapshot", "insert-block",
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
    "set-image-bytes", "patch-snapshot",
];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_document_mutation(snapshot: &mut SemioDocumentSnapshot, mutation: &SemioDocumentMutation) -> protocol::MutationOutcome<SemioDocumentDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Computes `mutation`'s own inverse against `base` — a thin wrapper around
/// `protocol::Mutation::inverse` so external Rust callers that cannot name this crate's private
/// `protocol` extern-crate item (the `📃️mutate-semio-document` test adapter, whose `inverse-<kind>`
/// scenarios need a mutation's own computed inverse) can still reach the inverse law that
/// [`apply_semio_document_mutation`] alone cannot. Same shape as `🧰️kit`'s
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

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioDocumentMutation, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    protocol::MutationOutcome::new(match this {
        SemioDocumentMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        SemioDocumentMutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioDocumentSnapshot, SemioDocumentMutation>>::diff(patch, base),
        SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path, block }) => wrap_body_diff(path, DocBlockLeaf::Inserted(block.clone())),
        SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path }) => wrap_body_diff(path, DocBlockLeaf::Removed),
        SemioDocumentMutation::SetBlockContent(set_block_content::SetBlockContent { path, block }) => match block_at(base, path) {
            Some(old) => match diff_block(old, block) {
                Some(d) => wrap_body_diff(path, DocBlockLeaf::Modified(d)),
                None => SemioDocumentDiff::default(),
            },
            None => SemioDocumentDiff::default(),
        },
        SemioDocumentMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { path, style_id }) => match block_at(base, path) {
            Some(DocBlock::Paragraph { style_id: old, .. }) if old != style_id => wrap_body_diff(path, DocBlockLeaf::Modified(DocBlockDiff::Paragraph(DocParagraphDiff { style_id: Some(style_id.clone()), runs: None }))),
            _ => SemioDocumentDiff::default(),
        },
        SemioDocumentMutation::SetHeadingLevel(set_heading_level::SetHeadingLevel { path, level }) => match block_at(base, path) {
            Some(DocBlock::Heading { level: old, .. }) if old != level => wrap_body_diff(path, DocBlockLeaf::Modified(DocBlockDiff::Heading(DocHeadingDiff { level: Some(*level), style_id: None, runs: None }))),
            _ => SemioDocumentDiff::default(),
        },
        SemioDocumentMutation::SetListOrdered(set_list_ordered::SetListOrdered { path, ordered }) => match block_at(base, path) {
            Some(DocBlock::List { ordered: old, .. }) if old != ordered => wrap_body_diff(path, DocBlockLeaf::Modified(DocBlockDiff::List(crate::standards::v1::subsets::document::schema::diff::DocListDiff { ordered: Some(*ordered), items: None }))),
            _ => SemioDocumentDiff::default(),
        },
        SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path, run_index, text }) => {
            let Some(block) = block_at(base, path) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
            let Some(runs) = runs_of(block) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
            let Some(run) = runs.get(*run_index) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
            if &run.text == text {
                return protocol::MutationOutcome::new(SemioDocumentDiff::default());
            }
            let rd: RunsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *run_index, diff: DocRunDiff { text: Some(text.clone()), style: None } }], ..Default::default() };
            match wrap_runs_diff(block, rd) {
                Some(bd) => wrap_body_diff(path, DocBlockLeaf::Modified(bd)),
                None => SemioDocumentDiff::default(),
            }
        }
        SemioDocumentMutation::SetRunStyle(set_run_style::SetRunStyle { path, run_index, style }) => {
            let Some(block) = block_at(base, path) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
            let Some(runs) = runs_of(block) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
            let Some(run) = runs.get(*run_index) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
            if &run.style == style {
                return protocol::MutationOutcome::new(SemioDocumentDiff::default());
            }
            let style_diff = crate::standards::v1::subsets::document::schema::diff::RunStyleDiff {
                bold: Some(style.bold),
                italic: Some(style.italic),
                underline: Some(style.underline),
                size: Some(style.size),
                font: Some(style.font.clone()),
                color: Some(style.color.clone()),
                link: Some(style.link.clone()),
            };
            let rd: RunsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *run_index, diff: DocRunDiff { text: None, style: Some(style_diff) } }], ..Default::default() };
            match wrap_runs_diff(block, rd) {
                Some(bd) => wrap_body_diff(path, DocBlockLeaf::Modified(bd)),
                None => SemioDocumentDiff::default(),
            }
        }
        SemioDocumentMutation::SetImageBlock(set_image_block::SetImageBlock { path, image_id, alt, width, height }) => match block_at(base, path) {
            Some(old @ DocBlock::Image { .. }) => {
                let new = DocBlock::Image { image_id: image_id.clone(), alt: alt.clone(), width: *width, height: *height };
                match diff_block(old, &new) {
                    Some(d) => wrap_body_diff(path, DocBlockLeaf::Modified(d)),
                    None => SemioDocumentDiff::default(),
                }
            }
            _ => SemioDocumentDiff::default(),
        },
        SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style }) => {
            SemioDocumentDiff { styles: Some(crate::standards::v1::subsets::document::schema::diff::StylesDiff { added: vec![style.clone()], ..Default::default() }), images: None, blocks: None }
        }
        SemioDocumentMutation::RemoveStyle(remove_style::RemoveStyle { id }) => {
            SemioDocumentDiff { styles: Some(crate::standards::v1::subsets::document::schema::diff::StylesDiff { removed: vec![id.clone()], ..Default::default() }), images: None, blocks: None }
        }
        SemioDocumentMutation::SetStyleName(set_style_name::SetStyleName { id, name }) => match style_at(base, id) {
            Some(old) if &old.name != name => SemioDocumentDiff {
                styles: Some(crate::standards::v1::subsets::document::schema::diff::StylesDiff {
                    modified: vec![crate::standards::v1::subsets::base::schema::triples::NamedModified { key: id.clone(), diff: crate::standards::v1::subsets::document::schema::diff::DocStyleDiff { name: Some(name.clone()), based_on: None } }],
                    ..Default::default()
                }),
                images: None,
                blocks: None,
            },
            _ => SemioDocumentDiff::default(),
        },
        SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id, based_on }) => match style_at(base, id) {
            Some(old) if &old.based_on != based_on => SemioDocumentDiff {
                styles: Some(crate::standards::v1::subsets::document::schema::diff::StylesDiff {
                    modified: vec![crate::standards::v1::subsets::base::schema::triples::NamedModified { key: id.clone(), diff: crate::standards::v1::subsets::document::schema::diff::DocStyleDiff { name: None, based_on: Some(based_on.clone()) } }],
                    ..Default::default()
                }),
                images: None,
                blocks: None,
            },
            _ => SemioDocumentDiff::default(),
        },
        SemioDocumentMutation::InsertImage(insert_image::InsertImage { image }) => {
            SemioDocumentDiff { styles: None, images: Some(crate::standards::v1::subsets::document::schema::diff::ImagesDiff { added: vec![image.clone()], ..Default::default() }), blocks: None }
        }
        SemioDocumentMutation::RemoveImage(remove_image::RemoveImage { id }) => {
            SemioDocumentDiff { styles: None, images: Some(crate::standards::v1::subsets::document::schema::diff::ImagesDiff { removed: vec![id.clone()], ..Default::default() }), blocks: None }
        }
        SemioDocumentMutation::SetImageBytes(set_image_bytes::SetImageBytes { id, mime, bytes }) => match image_at(base, id) {
            Some(old) if &old.mime != mime || &old.bytes != bytes => SemioDocumentDiff {
                styles: None,
                images: Some(crate::standards::v1::subsets::document::schema::diff::ImagesDiff {
                    modified: vec![crate::standards::v1::subsets::base::schema::triples::NamedModified {
                        key: id.clone(),
                        diff: crate::standards::v1::subsets::document::schema::diff::DocImageDiff { mime: Some(mime.clone()), bytes: Some(bytes.clone()) },
                    }],
                    ..Default::default()
                }),
                blocks: None,
            },
            _ => SemioDocumentDiff::default(),
        },
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioDocumentMutation, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    Ok({
    match this {
        SemioDocumentMutation::SetSnapshot(_) => vec![SemioDocumentMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        SemioDocumentMutation::PatchSnapshot(patch) => return Ok(<patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioDocumentSnapshot, SemioDocumentMutation>>::inverse(patch, base)?),
        SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path, .. }) => vec![SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path: path.clone() })],
        SemioDocumentMutation::RemoveBlock(remove_block::RemoveBlock { path }) => match block_at(base, path) {
            Some(block) => vec![SemioDocumentMutation::InsertBlock(insert_block::InsertBlock { path: path.clone(), block: block.clone() })],
            None => Vec::new(),
        },
        SemioDocumentMutation::SetBlockContent(set_block_content::SetBlockContent { path, .. }) => match block_at(base, path) {
            Some(block) => vec![SemioDocumentMutation::SetBlockContent(set_block_content::SetBlockContent { path: path.clone(), block: block.clone() })],
            None => Vec::new(),
        },
        SemioDocumentMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { path, .. }) => match block_at(base, path) {
            Some(DocBlock::Paragraph { style_id, .. }) => vec![SemioDocumentMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { path: path.clone(), style_id: style_id.clone() })],
            _ => Vec::new(),
        },
        SemioDocumentMutation::SetHeadingLevel(set_heading_level::SetHeadingLevel { path, .. }) => match block_at(base, path) {
            Some(DocBlock::Heading { level, .. }) => vec![SemioDocumentMutation::SetHeadingLevel(set_heading_level::SetHeadingLevel { path: path.clone(), level: *level })],
            _ => Vec::new(),
        },
        SemioDocumentMutation::SetListOrdered(set_list_ordered::SetListOrdered { path, .. }) => match block_at(base, path) {
            Some(DocBlock::List { ordered, .. }) => vec![SemioDocumentMutation::SetListOrdered(set_list_ordered::SetListOrdered { path: path.clone(), ordered: *ordered })],
            _ => Vec::new(),
        },
        SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path, run_index, .. }) => match block_at(base, path).and_then(runs_of).and_then(|r| r.get(*run_index)) {
            Some(run) => vec![SemioDocumentMutation::SetRunText(set_run_text::SetRunText { path: path.clone(), run_index: *run_index, text: run.text.clone() })],
            None => Vec::new(),
        },
        SemioDocumentMutation::SetRunStyle(set_run_style::SetRunStyle { path, run_index, .. }) => match block_at(base, path).and_then(runs_of).and_then(|r| r.get(*run_index)) {
            Some(run) => vec![SemioDocumentMutation::SetRunStyle(set_run_style::SetRunStyle { path: path.clone(), run_index: *run_index, style: run.style.clone() })],
            None => Vec::new(),
        },
        SemioDocumentMutation::SetImageBlock(set_image_block::SetImageBlock { path, .. }) => match block_at(base, path) {
            Some(DocBlock::Image { image_id, alt, width, height }) => {
                vec![SemioDocumentMutation::SetImageBlock(set_image_block::SetImageBlock { path: path.clone(), image_id: image_id.clone(), alt: alt.clone(), width: *width, height: *height })]
            }
            _ => Vec::new(),
        },
        SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style }) => vec![SemioDocumentMutation::RemoveStyle(remove_style::RemoveStyle { id: style.id.clone() })],
        SemioDocumentMutation::RemoveStyle(remove_style::RemoveStyle { id }) => match style_at(base, id) {
            Some(style) => vec![SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style: style.clone() })],
            None => Vec::new(),
        },
        SemioDocumentMutation::SetStyleName(set_style_name::SetStyleName { id, .. }) => match style_at(base, id) {
            Some(style) => vec![SemioDocumentMutation::SetStyleName(set_style_name::SetStyleName { id: id.clone(), name: style.name.clone() })],
            None => Vec::new(),
        },
        SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id, .. }) => match style_at(base, id) {
            Some(style) => vec![SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: id.clone(), based_on: style.based_on.clone() })],
            None => Vec::new(),
        },
        SemioDocumentMutation::InsertImage(insert_image::InsertImage { image }) => vec![SemioDocumentMutation::RemoveImage(remove_image::RemoveImage { id: image.id.clone() })],
        SemioDocumentMutation::RemoveImage(remove_image::RemoveImage { id }) => match image_at(base, id) {
            Some(image) => vec![SemioDocumentMutation::InsertImage(insert_image::InsertImage { image: image.clone() })],
            None => Vec::new(),
        },
        SemioDocumentMutation::SetImageBytes(set_image_bytes::SetImageBytes { id, .. }) => match image_at(base, id) {
            Some(image) => vec![SemioDocumentMutation::SetImageBytes(set_image_bytes::SetImageBytes { id: id.clone(), mime: image.mime.clone(), bytes: image.bytes.clone() })],
            None => Vec::new(),
        },
    }

    })
}
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
        SemioDocumentMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        SemioDocumentMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: crate::standards::v1::subsets::document::schema::diff::snapshot_b() }),
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
        SemioDocumentMutation::InsertStyle(insert_style::InsertStyle { style: DocStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) } }),
        SemioDocumentMutation::RemoveStyle(remove_style::RemoveStyle { id: "Normal".into() }),
        SemioDocumentMutation::SetStyleName(set_style_name::SetStyleName { id: "Normal".into(), name: "Body Text".into() }),
        SemioDocumentMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Other".into()) }),
        SemioDocumentMutation::InsertImage(insert_image::InsertImage { image: DocImage { id: "img2".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] } }),
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

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/📋️bolds/🦀️.rs"]
mod set_snapshot_bolds_the_body_paragraph_and_finalizes_its_copy;
//#endregion 🧪️FixtureCases
