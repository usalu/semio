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

use crate::artifacts::md::schema::diff::navigate_container;
pub use crate::artifacts::md::schema::diff::MdPathStep;
use crate::artifacts::md::schema::diff::{dec_block, dec_block_list, dec_inline_list, dec_str, enc_block, enc_block_list, enc_inline_list, enc_str, parse_usize, split_top_level, strip_brackets};
use crate::artifacts::md::schema::diff::{dec_block_bin, dec_block_list_bin, dec_inline_list_bin, enc_block_bin, enc_block_list_bin, enc_inline_list_bin, read_str_bin, write_str_bin};
use crate::artifacts::md::schema::diff::{diff_at_path, diff_set_snapshot, MdBlockDiff, MdBlocksLeafDiff, MdDiff};
use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
use crate::artifacts::md::MdSnapshot;
use protocol::{Mutation, OpText};

//#region 🔖️Mutations
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
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "➕insert-block/🦀️.rs"]
pub mod insert_block;
#[path = "➖remove-block/🦀️.rs"]
pub mod remove_block;
#[path = "🔁replace-block/🦀️.rs"]
pub mod replace_block;
#[path = "✏set-inlines/🦀️.rs"]
pub mod set_inlines;
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
/// standalone `🧪️tests/mutate-md-commonmark/🦀️.rs` test adapter carries its OWN,
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
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
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
pub(crate) fn agg_inverse(this: &MdMutation, base: &MdSnapshot) -> Vec<MdMutation> {
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
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `MdMutation` (`#[derive(dsl::DslOps)]` confirmed
/// rejected above) — reuses `MdDiff`'s `pub(crate)` grammar primitives (`enc_block`/
/// `enc_inline_list`/`split_top_level`/...) rather than duplicating them a second time in this
/// file, same intra-artifact-reuse pattern `SvgMutation` uses for `SvgDiff`'s primitives. Grammar:
/// `keyword arg=value ...` (space-separated, same shape the derive's own handcrafted-wrapper
/// convention uses), one match arm per variant (no `DslVariants` scaffolding available since
/// nothing here derives it). `MdPathStep` gets tag range Y-Z (see `MdDiff`'s region doc comment for
/// the full tag-vocabulary table).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_path_step(step: &MdPathStep) -> String {
    match step {
        MdPathStep::BlockQuote { index } => format!("Y[{index}]"),
        MdPathStep::ListItem { index, item } => format!("Z[{index},{item}]"),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_path_step(s: &str) -> Result<MdPathStep, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "Y" => Ok(MdPathStep::BlockQuote { index: parse_usize(inner)? }),
        "Z" => {
            let parts = split_top_level(inner, ',');
            let [index, item] = parts.as_slice() else { return Err(format!("list item path step: expected 2 fields, got {}", parts.len())) };
            Ok(MdPathStep::ListItem { index: parse_usize(index)?, item: parse_usize(item)? })
        }
        other => Err(format!("path step: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_path(path: &[MdPathStep]) -> String {
    format!("[{}]", path.iter().map(enc_path_step).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_path(s: &str) -> Result<Vec<MdPathStep>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_path_step).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_md_snapshot(s: &MdSnapshot) -> String {
    format!("[{},{}]", enc_str(&s.schema), enc_block_list(&s.blocks))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_md_snapshot(s: &str) -> Result<MdSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, blocks] = parts.as_slice() else { return Err(format!("md snapshot: expected 2 fields, got {}", parts.len())) };
    Ok(MdSnapshot { schema: dec_str(schema)?, blocks: dec_block_list(blocks)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_md_mutation(m: &MdMutation) -> String {
    match m {
        MdMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_md_snapshot(snapshot)),
        MdMutation::InsertBlock(insert_block::InsertBlock { path, index, block }) => format!("insert-block path={} index={index} block={}", enc_path(path), enc_block(block)),
        MdMutation::RemoveBlock(remove_block::RemoveBlock { path, index }) => format!("remove-block path={} index={index}", enc_path(path)),
        MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path, index, block }) => format!("replace-block path={} index={index} block={}", enc_path(path), enc_block(block)),
        MdMutation::SetInlines(set_inlines::SetInlines { path, index, inlines }) => format!("set-inlines path={} index={index} inlines={}", enc_path(path), enc_inline_list(inlines)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_md_mutation(line: &str) -> Result<MdMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("md mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("md mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(MdMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_md_snapshot(arg("snapshot")?)? })),
        "insert-block" => Ok(MdMutation::InsertBlock(insert_block::InsertBlock { path: dec_path(arg("path")?)?, index: usize_arg("index")?, block: dec_block(arg("block")?)? })),
        "remove-block" => Ok(MdMutation::RemoveBlock(remove_block::RemoveBlock { path: dec_path(arg("path")?)?, index: usize_arg("index")? })),
        "replace-block" => Ok(MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path: dec_path(arg("path")?)?, index: usize_arg("index")?, block: dec_block(arg("block")?)? })),
        "set-inlines" => Ok(MdMutation::SetInlines(set_inlines::SetInlines { path: dec_path(arg("path")?)?, index: usize_arg("index")?, inlines: dec_inline_list(arg("inlines")?)? })),
        other => Err(format!("md mutation: unknown keyword {other:?}")),
    }
}

impl OpText for MdMutation {
    fn print_op(&self) -> String {
        print_md_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_md_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ P2-FG1: mutation-specific real binary primitives backing the upgraded `OpBinary` impl below
/// — reuses `MdDiff`'s `pub(crate)` recursive `enc_block_bin`/`enc_inline_list_bin`/`write_str_bin`/
/// `write_option_bin` primitives (`../../🔺️diff/🦀️.rs`, imported above) for the SHARED
/// `MdBlock`/`MdInline` shape (same intra-artifact-reuse split the TEXT codec above already uses),
/// only `MdSnapshot`/`MdPathStep`'s own binary shape is genuinely new here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_snapshot_bin(s: &MdSnapshot, out: &mut Vec<u8>) {
    write_str_bin(out, &s.schema);
    enc_block_list_bin(&s.blocks, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<MdSnapshot, String> {
    let schema = read_str_bin(reader)?;
    let blocks = dec_block_list_bin(reader)?;
    Ok(MdSnapshot { schema, blocks })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_path_step_bin(step: &MdPathStep, out: &mut Vec<u8>) {
    match step {
        MdPathStep::BlockQuote { index } => {
            out.push(0);
            store::pack_rt::write_varint_u64(out, *index as u64);
        }
        MdPathStep::ListItem { index, item } => {
            out.push(1);
            store::pack_rt::write_varint_u64(out, *index as u64);
            store::pack_rt::write_varint_u64(out, *item as u64);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_path_step_bin(reader: &mut store::ByteReader<'_>) -> Result<MdPathStep, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(MdPathStep::BlockQuote { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize }),
        1 => {
            let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
            let item = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
            Ok(MdPathStep::ListItem { index, item })
        }
        other => Err(format!("path step binary: unknown tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_path_bin(path: &[MdPathStep], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, path.len() as u64);
    for step in path {
        enc_path_step_bin(step, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_path_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<MdPathStep>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_path_step_bin(reader)).collect()
}
//#endregion 🔖️OpBinaryCodec

/// 🧪️ P2-FG1: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the `MdMutation`
/// variant ordinal, same 1-5 order `print_md_mutation`'s own keyword match uses (0 was
/// `NoMutation`'s, dropped by the `26/08/29/S-END-TO-END` mutation-leaf migration; the remaining
/// tags are left as they were rather than renumbered down, since nothing needs them contiguous).
impl protocol::OpBinary for MdMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            MdMutation::SetSnapshot(_) => 1,
            MdMutation::InsertBlock(_) => 2,
            MdMutation::RemoveBlock(_) => 3,
            MdMutation::ReplaceBlock(_) => 4,
            MdMutation::SetInlines(_) => 5,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            MdMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_snapshot_bin(snapshot, &mut out),
            MdMutation::InsertBlock(insert_block::InsertBlock { path, index, block }) => {
                enc_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_block_bin(block, &mut out);
            }
            MdMutation::RemoveBlock(remove_block::RemoveBlock { path, index }) => {
                enc_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path, index, block }) => {
                enc_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_block_bin(block, &mut out);
            }
            MdMutation::SetInlines(set_inlines::SetInlines { path, index, inlines }) => {
                enc_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_inline_list_bin(inlines, &mut out);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            1 => {
                let snapshot = dec_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(MdMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            2 => {
                let path = dec_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(MdMutation::InsertBlock(insert_block::InsertBlock { path, index, block }))
            }
            3 => {
                let path = dec_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(MdMutation::RemoveBlock(remove_block::RemoveBlock { path, index }))
            }
            4 => {
                let path = dec_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(MdMutation::ReplaceBlock(replace_block::ReplaceBlock { path, index, block }))
            }
            5 => {
                let path = dec_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let inlines = dec_inline_list_bin(&mut reader).map_err(|e| malformed("op inlines", reader.position(), e))?;
                Ok(MdMutation::SetInlines(set_inlines::SetInlines { path, index, inlines }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
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
    let base = MdSnapshot { schema: crate::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "hi".into() }] }] };
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
mod op_codec_tests {
    use super::*;
    use protocol::OpBinary;

    /// 🧪️ F6/P2-FG1: `OpText`/`OpBinary` round-trip laws for the hand-rolled `MdMutation` grammar —
    /// see `demo_mutation_cases()`'s own doc comment for exactly what each case exercises.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = MdMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = MdMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }

    /// 🧪️ `kinds_match_enum_variants_and_catalog`: `KINDS` lists every `MdMutation` variant
    /// exactly once (the `match` below has no wildcard arm, so a new variant fails to compile
    /// here first) AND matches the mutation catalog this subset's `🔣️oracle.json`
    /// declares, in the same order — the framework's completeness gate reads that JSON, never this
    /// enum, so this test is the only thing tying the two declarations together.
    #[semio_framework_async_macros::async_test]
    async fn kinds_match_enum_variants_and_catalog() {
        // 🚫️async: E1 pure inherent helper, no I/O — see R9
        fn kebab_of(mutation: &MdMutation) -> &'static str {
            match mutation {
                MdMutation::SetSnapshot(_) => "set-snapshot",
                MdMutation::InsertBlock(_) => "insert-block",
                MdMutation::RemoveBlock(_) => "remove-block",
                MdMutation::ReplaceBlock(_) => "replace-block",
                MdMutation::SetInlines(_) => "set-inlines",
            }
        }
        let variant_kinds: std::collections::BTreeSet<&str> = demo_mutation_cases().iter().map(kebab_of).collect();
        let declared_kinds: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
        assert_eq!(variant_kinds, declared_kinds, "KINDS must list every MdMutation variant exactly once");

        // 🧭️ Containment, not equality: the manifest's `kinds` ALSO carries `no-mutation`, the
        // identity-probe row `🧪️tests/mutate-md-commonmark/🦀️.rs` registers directly
        // (real `mutate-no-mutation`/`inverse-no-mutation` scenario rows in that case's own feature
        // file) — it names no `MdMutation` variant (dropped by the `26/08/29/S-END-TO-END`
        // mutation-leaf migration: `no` is not an approved semantic verb) and so cannot appear in
        // `KINDS`, which is exhaustively derived from the enum's own variants above.
        let manifest: serde_json::Value = serde_json::from_str(include_str!("../../🔣️oracle.json")).expect("valid catalog JSON");
        let catalog_kinds: Vec<&str> = manifest["mutationCatalogs"][0]["kinds"].as_array().expect("mutationCatalogs[0].kinds array").iter().map(|value| value.as_str().expect("kind is a string")).collect();
        for kind in KINDS {
            assert!(catalog_kinds.contains(kind), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/demotes-the-tower-heading-to-level-3/🦀️.rs"]
    mod tests_set_snapshot_demotes_the_tower_heading_to_level_3;
}
//#endregion 🧪️FixtureTests
