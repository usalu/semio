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



















use crate::schema::diff::{diff_at_path, MdBlockDiff, MdBlocksLeafDiff, MdDiff};
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
#[path = "✂️splice-source/🦀️.rs"]
pub mod splice_source;
/// 📐️ Typed content mutation for `stdio.md`. Every `path`-carrying variant addresses the
/// CONTAINER (the `Vec<MdBlock>` -- top level, a block-quote's `blocks`, or a list item's
/// content) the mutation's `index` lives in; `path == []` addresses the top-level `blocks`.
/// 🧪️ F6: `#[derive(dsl::DslOps)]` on this enum is structurally blocked the SAME way
/// `SvgMutation`'s was — `InsertBlock`/`ReplaceBlock`'s `block: MdBlock` /
/// `SetInlines`'s `inlines: Vec<MdInline>` carry an enum-shaped payload DIRECTLY as a leaf
/// field, not just via a nested snapshot — the mutation-side twin of the diff-side blocker cited on
/// `MdDiff`'s own doc comment. `OpText`/`OpBinary` hand-rolled below, reusing `MdDiff`'s
/// `pub(crate)` grammar primitives (`enc_block`/`enc_inline_list`/`split_top_level`/...).
//#region 🔖️Leaves
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = MdSnapshot, diff = MdDiff, schema = "MdMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum MdMutation {
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
    /// ✂️ Applies the ranges an editor made to the CommonMark source text (the blocks rendered, blank-line separated): the blocks the
    /// ranges touch and their neighbours are reparsed from the edited source, every other block stays.
    SpliceSource(splice_source::SpliceSource),
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
pub const KINDS: &[&str] = &["insert-block", "remove-block", "replace-block", "set-inlines", "splice-source"];
//#endregion 🔖️Kinds



//#endregion 🔖️Apply

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
        MdMutation::SpliceSource(splice_source::SpliceSource { splices: vec![splice_source::SourceSplice { offset: 1, delete: 1, insert: "ho".into() }, splice_source::SourceSplice { offset: 2, delete: 0, insert: "\n\n# t".into() }] }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️op-codec/🦀️.rs"]
mod op_codec_tests;
//#endregion 🧪️Tests



#[cfg(test)]
use protocol::{OpText};
