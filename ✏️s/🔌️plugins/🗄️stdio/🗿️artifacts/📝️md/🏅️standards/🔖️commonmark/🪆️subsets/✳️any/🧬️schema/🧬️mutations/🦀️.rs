//! 🧬️ MdMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, path/index-aware.
//!
//! Leaf-per-variant shape mirrored from `🖼️tiff`'s `TiffBaselineMutation` (ticket
//! `26/08/29/S-END-TO-END`): `NoMutation` was dropped (`#[derive(dsl::Mutations)]` requires every
//! variant to wrap exactly one leaf payload and a unit variant wraps none; `no` is not an approved
//! semantic verb either), and every remaining variant now wraps its own `dsl::MutationLeaf` struct
//! instead of carrying its fields as a struct-literal directly. `#[value(tag = "mutation", ...)]`
//! is kept so the wire shape this artifact's committed fixtures depend on stays byte-for-byte
//! identical — serde's internally-tagged representation supports a newtype variant wrapping a plain
//! struct. `OpText`/`OpBinary` remain hand-rolled below (F6: `#[derive(dsl::DslOps)]` is
//! structurally blocked, see that region's own doc comment), so only each match arm's pattern head
//! changed there too, from `MdMutation::Variant { a, b }` to `MdMutation::Variant(variant_mod::
//! Variant { a, b })`.

use crate::schema::diff::navigate_container;
pub use crate::schema::diff::MdPathStep;



















use crate::schema::diff::{diff_at_path, diff_set_snapshot, MdBlockDiff, MdBlocksLeafDiff, MdDiff};
use crate::schema::snapshot::{MdBlock, MdInline};
use crate::MdSnapshot;
use protocol::{Mutation};

//#region 🔖️Mutations
#[path = "➕insert-block/🦀️.rs"]
pub mod insert_block;
#[path = "➖remove-block/🦀️.rs"]
pub mod remove_block;
#[path = "🔁replace-block/🦀️.rs"]
pub mod replace_block;
#[path = "✏️set-inlines/🦀️.rs"]
pub mod set_inlines;
/// 📐️ Typed content mutation for `stdio.md`. Every `path`-carrying variant addresses the
/// CONTAINER (the `Vec<MdBlock>` -- top level, a block-quote's `blocks`, or a list item's
/// content) the mutation's `index` lives in; `path == []` addresses the top-level `blocks`.
/// 🧪️ F6: `#[derive(dsl::DslOps)]` on this enum is structurally blocked the SAME way
/// `SvgMutation`'s was — `SetSnapshot`'s `snapshot: MdSnapshot` recursively contains `MdBlock`
/// (a genuine data-carrying enum, no `DslField` impl, same `E0277` shape `SvgNodeDiff`/`XmlNode`
/// hit via `SvgSnapshot`), and `InsertBlock`/`ReplaceBlock`'s `block: MdBlock` /
/// `SetInlines`'s `inlines: Vec<MdInline>` carry an enum-shaped payload DIRECTLY as a leaf
/// field, not just via a nested snapshot — the mutation-side twin of the diff-side blocker cited on
/// `MdDiff`'s own doc comment. `OpText`/`OpBinary` hand-rolled below, reusing `MdDiff`'s
/// `pub(crate)` grammar primitives (`enc_block`/`enc_inline_list`/`split_top_level`/...).
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = MdSnapshot, diff = MdDiff, schema = "MdMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum MdMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// ➕️ Inserts `block` at `index` within the container addressed by `path`.
    InsertBlock(insert_block::InsertBlock),
    /// ➖️ Removes the block at `index` within the container addressed by `path`.
    RemoveBlock(remove_block::RemoveBlock),
    /// 🔁 Wholesale-replaces the block at `index` (documented "your call" per the brief: a
    /// generic full-block replace instead of a per-field mutation for every one of `MdBlock`'s 7
    /// variants -- `SetInlines` below covers the one field (`inlines`) that's actually common and
    /// worth its own targeted mutation; every other field-level edit goes through `ReplaceBlock`).
    ReplaceBlock(replace_block::ReplaceBlock),
    /// ✏️ Whole-value replaces the `inlines` of the `Heading`/`Paragraph` block at `index`
    /// (`MdInline` is a weak entity -- recipe: whole-value replaced, never sub-diffed). A
    /// graceful no-op (empty diff) if the addressed block isn't one of those two kinds --
    /// documented degrade-gracefully behavior, never a panic.
    SetInlines(set_inlines::SetInlines),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🗂️ Kebab-case spelling of every `MdMutation` variant, declaration order. Every entry also
/// appears in this subset's `🔣️oracle.json` mutation catalog (`md-commonmark-any`);
/// `kinds_match_enum_variants_and_catalog` below is what keeps the two lists honest. The
/// standalone `🧪️tests/📝️mutate-md-commonmark/🦀️.rs` test adapter carries its OWN,
/// separately-declared `no-mutation` identity-probe scenario on top of these five real kinds — it
/// names no `MdMutation` variant (dropped by the `26/08/29/S-END-TO-END` mutation-leaf migration:
/// `no` is not an approved semantic verb) and is handled directly by that adapter's `mutate`/
/// `inverse` functions rather than through this vocabulary.
pub const KINDS: &[&str] = &["set-snapshot", "insert-block", "remove-block", "replace-block", "set-inlines"];
//#endregion 🔖️Kinds

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_md_mutation(snapshot: &mut MdSnapshot, mutation: &MdMutation) -> protocol::MutationOutcome<MdDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Apply

//#region 🔖️MutationTrait
/// 🧷️ Lifted verbatim from the former `impl Mutation<MdSnapshot> for MdMutation`'s own `diff` body
/// — only each match arm's pattern head changed, from `MdMutation::Variant { .. }` to
/// `MdMutation::Variant(variant_mod::Variant { .. })`, to destructure the leaf payload each variant
/// now wraps.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn agg_diff(this: &MdMutation, base: &MdSnapshot) -> protocol::MutationOutcome<MdDiff> {
    protocol::MutationOutcome::new(match this {
        MdMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        MdMutation::InsertBlock(insert_block::InsertBlock { path, index, block }) => diff_at_path(path, *index, MdBlocksLeafDiff::Added(block.clone())),
        MdMutation::RemoveBlock(remove_block::RemoveBlock { path, index }) => diff_at_path(path, *index, MdBlocksLeafDiff::Removed),
        MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path, index, block }) => diff_at_path(path, *index, MdBlocksLeafDiff::Modified(MdBlockDiff::Replace { block: block.clone() })),
        MdMutation::SetInlines(set_inlines::SetInlines { path, index, inlines }) => match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)) {
            Some(MdBlock::Heading { .. }) => diff_at_path(path, *index, MdBlocksLeafDiff::Modified(MdBlockDiff::Heading { level: None, inlines: Some(inlines.clone()) })),
            Some(MdBlock::Paragraph { .. }) => diff_at_path(path, *index, MdBlocksLeafDiff::Modified(MdBlockDiff::Paragraph { inlines: Some(inlines.clone()) })),
            _ => MdDiff::default(),
        },
    })
}

/// ↩️ Lifted verbatim from the former `impl Mutation<MdSnapshot> for MdMutation`'s own `inverse`
/// body. A mutation with nothing to invert against now inverts to the EMPTY vec (`NoMutation`,
/// dropped by this migration, used to carry this case as a no-op sentinel; there is nothing to
/// undo, so there is nothing to return).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn agg_inverse(this: &MdMutation, base: &MdSnapshot) -> Result<Vec<MdMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match this {
        MdMutation::SetSnapshot(_) => vec![MdMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        MdMutation::InsertBlock(insert_block::InsertBlock { path, index, .. }) => vec![MdMutation::RemoveBlock(remove_block::RemoveBlock { path: path.clone(), index: *index })],
        MdMutation::RemoveBlock(remove_block::RemoveBlock { path, index }) => match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)).cloned() {
            Some(block) => vec![MdMutation::InsertBlock(insert_block::InsertBlock { path: path.clone(), index: *index, block })],
            None => Vec::new(),
        },
        MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path, index, .. }) => match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)).cloned() {
            Some(block) => vec![MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path: path.clone(), index: *index, block })],
            None => Vec::new(),
        },
        MdMutation::SetInlines(set_inlines::SetInlines { path, index, .. }) => {
            let original = match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)) {
                Some(MdBlock::Heading { inlines, .. }) => Some(inlines.clone()),
                Some(MdBlock::Paragraph { inlines }) => Some(inlines.clone()),
                _ => None,
            };
            match original {
                Some(inlines) => vec![MdMutation::SetInlines(set_inlines::SetInlines { path: path.clone(), index: *index, inlines })],
                None => Vec::new(),
            }
        }
    }

    })())
}
//#endregion 🔖️MutationTrait

//#region OpCodecs












//#region 🔖️OpBinaryCodec







//#endregion 🔖️OpBinaryCodec




//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `MdMutation` values (every variant, incl. `InsertBlock`/
/// `ReplaceBlock`'s bare `MdBlock` payload — a `List` block, so `enc_block`'s own recursive
/// `items: Vec<Vec<MdBlock>>` field gets exercised too — `SetInlines`'s `Vec<MdInline>` payload
/// (multiple inline kinds incl. nested `Emphasis`), and both `MdPathStep` variants incl. a
/// multi-step nested path) — the single source of truth reused by `op_text_binary_roundtrip_law`
/// below AND by `⚙️engine/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, so a new variant only needs adding here once.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<MdMutation> {
    let base = MdSnapshot { schema: crate::STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "hi".into() }] }] };
    let list_block = MdBlock::List { ordered: true, start: Some(2), tight: false, items: vec![vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "one".into() }] }], vec![MdBlock::BlockQuote { blocks: vec![MdBlock::ThematicBreak] }]] };
    vec![
        MdMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base }),
        MdMutation::InsertBlock(insert_block::InsertBlock { path: Vec::new(), index: 1, block: list_block.clone() }),
        MdMutation::InsertBlock(insert_block::InsertBlock { path: vec![MdPathStep::BlockQuote { index: 0 }], index: 0, block: MdBlock::HtmlBlock { raw: "<hr/>".into() } }),
        MdMutation::RemoveBlock(remove_block::RemoveBlock { path: Vec::new(), index: 0 }),
        MdMutation::RemoveBlock(remove_block::RemoveBlock { path: vec![MdPathStep::ListItem { index: 2, item: 1 }, MdPathStep::BlockQuote { index: 0 }], index: 3 }),
        MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path: Vec::new(), index: 0, block: MdBlock::CodeBlock { info: None, literal: "x".into() } }),
        MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path: vec![MdPathStep::BlockQuote { index: 1 }], index: 2, block: list_block }),
        MdMutation::SetInlines(set_inlines::SetInlines {
            path: Vec::new(),
            index: 0,
            inlines: vec![
                MdInline::Text { text: "hello".into() },
                MdInline::Emphasis { inlines: vec![MdInline::Strong { inlines: vec![MdInline::Text { text: "world".into() }] }] },
                MdInline::Link { text: vec![MdInline::Text { text: "l".into() }], url: "http://x".into(), title: None },
                MdInline::HardBreak,
            ],
        }),
        MdMutation::SetInlines(set_inlines::SetInlines { path: vec![MdPathStep::ListItem { index: 0, item: 0 }], index: 5, inlines: Vec::new() }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️op-codec/🦀️.rs"]
mod op_codec_tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests

#[cfg(test)]
use protocol::{OpText};
