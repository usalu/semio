//! 🔺️ MdDiff — handcrafted recursive tree diff. `blocks` is an index-keyed recursive triple over
//! the top-level `MdBlock` sequence; `MdBlockDiff` is shaped like the `MdBlock` it targets, with
//! `List.items` and `BlockQuote.blocks` nesting their OWN index-keyed triples (`MdListItemsDiff`
//! reuses this same `MdBlocksDiff`/`MdBlockDiff` shape recursively for each item's content, since
//! a list item's content IS a `Vec<MdBlock>` -- identical to the top level and to a block quote's
//! content). `MdInline` is treated as a WEAK entity throughout (recipe: weak entities are
//! whole-value replaced) -- every `inlines`/`text` field below is `Option<Vec<MdInline>>` or
//! `Option<String>`, never sub-diffed. Same xml/svg tree-diff pattern (`.🦑️repo/🎫️tickets/
//! 🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/🧬️schema-design.md`,
//! xml's own diff module is the direct template this file follows arm-for-arm).

use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
use crate::artifacts::md::MdSnapshot;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};
use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.md`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md.diff")]
pub struct MdDiff {
    /// 🌳 `None` = top-level block sequence unchanged; `Some(diff)` = index-keyed recursive
    /// triple over it.
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<MdBlocksDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️BlocksDiff
/// 🌳 Index-keyed, recursive block-sequence triple. `removed`/`modified` indices refer to BASE
/// state (descending removal order on apply); `added` indices refer to FINAL state (ascending
/// insert). Reused verbatim (same type) for `List.items[n]`'s content AND `BlockQuote.blocks` --
/// both are `Vec<MdBlock>`, exactly what this type diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MdBlocksDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<MdBlockModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<MdBlockAdded>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MdBlockModified {
    pub index: usize,
    pub diff: MdBlockDiff,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MdBlockAdded {
    pub index: usize,
    pub item: MdBlock,
}

/// 🌳 Per-block diff, shaped like the `MdBlock` it targets. `Replace` is the fallback for a
/// block-KIND change (e.g. `Paragraph` -> `Heading`) -- every other variant assumes the target
/// keeps its kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum MdBlockDiff {
    Heading {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        level: Option<u8>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        inlines: Option<Vec<MdInline>>,
    },
    Paragraph {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        inlines: Option<Vec<MdInline>>,
    },
    List {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ordered: Option<bool>,
        /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = start number cleared, `Some(Some(n))`
        /// = set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        start: Option<Option<u32>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tight: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        items: Option<MdListItemsDiff>,
    },
    CodeBlock {
        /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = info string cleared, `Some(Some(s))`
        /// = set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        info: Option<Option<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        literal: Option<String>,
    },
    BlockQuote {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blocks: Option<MdBlocksDiff>,
    },
    /// 🔳 `ThematicBreak` carries no fields -- this variant only appears via a kind-preserving
    /// `between`/`apply` no-op (two `ThematicBreak`s are always structurally equal, so `between`
    /// never actually constructs it; included for match exhaustiveness/API symmetry with every
    /// other `MdBlock` kind).
    ThematicBreak,
    HtmlBlock {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        raw: Option<String>,
    },
    /// 🔁 Wholesale block replace -- used when the block's KIND changes.
    Replace {
        block: MdBlock,
    },
}
//#endregion 🔖️BlocksDiff

//#region 🔖️ListItemsDiff
/// 🌳 Index-keyed triple over a `List`'s `items: Vec<Vec<MdBlock>>` -- each item's OWN content is
/// diffed with the same recursive `MdBlocksDiff` used everywhere else (a list item's content IS a
/// `Vec<MdBlock>`), so nested sub-lists/quotes inside an item fall out of the existing recursion
/// for free.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MdListItemsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<MdListItemModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<MdListItemAdded>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MdListItemModified {
    pub index: usize,
    pub diff: MdBlocksDiff,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MdListItemAdded {
    pub index: usize,
    pub item: Vec<MdBlock>,
}
//#endregion 🔖️ListItemsDiff

//#region 🔖️DiffAtPath
/// 🧭️ One descent step from a `Vec<MdBlock>` container down into a nested one.
/// `BlockQuote{index}` steps into the block-quote block at `index`'s own `blocks`;
/// `ListItem{index,item}` steps into the list block at `index`'s `items[item]`. Re-exported from
/// `crate::artifacts::md::schema::mutations` for ergonomic access -- kept here, not in the
/// mutations module, so this module never needs to depend on it (mutations already depends on
/// diff).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "step", rename_all = "camelCase")]
pub enum MdPathStep {
    BlockQuote { index: usize },
    ListItem { index: usize, item: usize },
}

/// 🍃 What a path-addressed mutation is doing at its final `index`: patch an existing block
/// in-place, insert a new one, or remove one.
pub enum MdBlocksLeafDiff {
    Modified(MdBlockDiff),
    Added(MdBlock),
    Removed,
}

/// 🧭️ Lowers a `leaf` diff at `index` within the container addressed by `path` (from the
/// document root) into a full `MdDiff`, nesting through `MdBlockModified`/`MdListItemModified`
/// chains from the root down to that depth. `path == []` addresses the top-level `blocks`
/// directly.
pub fn diff_at_path(path: &[MdPathStep], index: usize, leaf: MdBlocksLeafDiff) -> MdDiff {
    let inner = match leaf {
        MdBlocksLeafDiff::Modified(diff) => MdBlocksDiff { removed: Vec::new(), modified: vec![MdBlockModified { index, diff }], added: Vec::new() },
        MdBlocksLeafDiff::Added(block) => MdBlocksDiff { removed: Vec::new(), modified: Vec::new(), added: vec![MdBlockAdded { index, item: block }] },
        MdBlocksLeafDiff::Removed => MdBlocksDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() },
    };
    MdDiff { blocks: Some(wrap_blocks_diff(path, inner)) }
}

fn wrap_blocks_diff(path: &[MdPathStep], inner: MdBlocksDiff) -> MdBlocksDiff {
    let mut current = inner;
    for step in path.iter().rev() {
        current = match step {
            MdPathStep::BlockQuote { index } => MdBlocksDiff {
                removed: Vec::new(),
                added: Vec::new(),
                modified: vec![MdBlockModified { index: *index, diff: MdBlockDiff::BlockQuote { blocks: Some(current) } }],
            },
            MdPathStep::ListItem { index, item } => MdBlocksDiff {
                removed: Vec::new(),
                added: Vec::new(),
                modified: vec![MdBlockModified {
                    index: *index,
                    diff: MdBlockDiff::List {
                        ordered: None,
                        start: None,
                        tight: None,
                        items: Some(MdListItemsDiff {
                            removed: Vec::new(),
                            added: Vec::new(),
                            modified: vec![MdListItemModified { index: *item, diff: current }],
                        }),
                    },
                }],
            },
        };
    }
    current
}
//#endregion 🔖️DiffAtPath

//#region 🔖️Navigate
/// 🔎️ Walks `path` from `blocks`, returning the addressed container (the `Vec<MdBlock>` the
/// final `index` of a path-carrying mutation lives in). Graceful `None` on any out-of-range index
/// or kind mismatch (e.g. `ListItem` step into a non-`List` block), never a panic.
pub fn navigate_container<'a>(blocks: &'a [MdBlock], path: &[MdPathStep]) -> Option<&'a [MdBlock]> {
    let mut current = blocks;
    for step in path {
        current = match step {
            MdPathStep::BlockQuote { index } => match current.get(*index) {
                Some(MdBlock::BlockQuote { blocks }) => blocks.as_slice(),
                _ => return None,
            },
            MdPathStep::ListItem { index, item } => match current.get(*index) {
                Some(MdBlock::List { items, .. }) => items.get(*item)?.as_slice(),
                _ => return None,
            },
        };
    }
    Some(current)
}
//#endregion 🔖️Navigate

//#region 🔖️Apply
impl MutationDiff<MdSnapshot> for MdDiff {
    fn apply(&self, base: &MdSnapshot) -> MdSnapshot {
        let mut next = base.clone();
        if let Some(bd) = &self.blocks {
            next.blocks = apply_blocks_diff(&next.blocks, bd);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.blocks = match (self.blocks.take(), other.blocks) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_blocks_diff(a, b)),
        };
    }
}

fn apply_blocks_diff(blocks: &[MdBlock], diff: &MdBlocksDiff) -> Vec<MdBlock> {
    let mut slots: Vec<Option<MdBlock>> = blocks.iter().cloned().map(Some).collect();
    for m in &diff.modified {
        if let Some(Some(b)) = slots.get(m.index) {
            let patched = apply_block_diff(b, &m.diff);
            slots[m.index] = Some(patched);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    removed_sorted.dedup();
    for idx in removed_sorted {
        if idx < slots.len() {
            slots.remove(idx);
        }
    }
    let mut out: Vec<MdBlock> = slots.into_iter().flatten().collect();
    let mut additions: Vec<&MdBlockAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, add.item.clone());
    }
    out
}

fn apply_block_diff(block: &MdBlock, diff: &MdBlockDiff) -> MdBlock {
    match diff {
        MdBlockDiff::Replace { block: replacement } => replacement.clone(),
        MdBlockDiff::Heading { level, inlines } => match block {
            MdBlock::Heading { level: l, inlines: i } => {
                MdBlock::Heading { level: level.unwrap_or(*l), inlines: inlines.clone().unwrap_or_else(|| i.clone()) }
            }
            other => other.clone(),
        },
        MdBlockDiff::Paragraph { inlines } => match block {
            MdBlock::Paragraph { inlines: i } => MdBlock::Paragraph { inlines: inlines.clone().unwrap_or_else(|| i.clone()) },
            other => other.clone(),
        },
        MdBlockDiff::CodeBlock { info, literal } => match block {
            MdBlock::CodeBlock { info: i, literal: l } => MdBlock::CodeBlock {
                info: info.clone().unwrap_or_else(|| i.clone()),
                literal: literal.clone().unwrap_or_else(|| l.clone()),
            },
            other => other.clone(),
        },
        MdBlockDiff::HtmlBlock { raw } => match block {
            MdBlock::HtmlBlock { raw: r } => MdBlock::HtmlBlock { raw: raw.clone().unwrap_or_else(|| r.clone()) },
            other => other.clone(),
        },
        MdBlockDiff::ThematicBreak => MdBlock::ThematicBreak,
        MdBlockDiff::BlockQuote { blocks } => match block {
            MdBlock::BlockQuote { blocks: b } => {
                MdBlock::BlockQuote { blocks: match blocks { Some(d) => apply_blocks_diff(b, d), None => b.clone() } }
            }
            other => other.clone(),
        },
        MdBlockDiff::List { ordered, start, tight, items } => match block {
            MdBlock::List { ordered: o, start: s, tight: t, items: it } => MdBlock::List {
                ordered: ordered.unwrap_or(*o),
                start: start.clone().unwrap_or(*s),
                tight: tight.unwrap_or(*t),
                items: match items { Some(d) => apply_list_items_diff(it, d), None => it.clone() },
            },
            other => other.clone(),
        },
    }
}

fn apply_list_items_diff(items: &[Vec<MdBlock>], diff: &MdListItemsDiff) -> Vec<Vec<MdBlock>> {
    let mut slots: Vec<Option<Vec<MdBlock>>> = items.iter().cloned().map(Some).collect();
    for m in &diff.modified {
        if let Some(Some(b)) = slots.get(m.index) {
            let patched = apply_blocks_diff(b, &m.diff);
            slots[m.index] = Some(patched);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    removed_sorted.dedup();
    for idx in removed_sorted {
        if idx < slots.len() {
            slots.remove(idx);
        }
    }
    let mut out: Vec<Vec<MdBlock>> = slots.into_iter().flatten().collect();
    let mut additions: Vec<&MdListItemAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, add.item.clone());
    }
    out
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<MdSnapshot> for MdDiff {
    fn inverse(&self, base: &MdSnapshot) -> Self {
        MdDiff { blocks: self.blocks.as_ref().map(|d| inverse_blocks_diff(&base.blocks, d)) }
    }

    fn between(base: &MdSnapshot, other: &MdSnapshot) -> Self {
        MdDiff { blocks: between_blocks(&base.blocks, &other.blocks) }
    }

    fn is_empty(&self) -> bool {
        self.blocks.is_none()
    }
}

fn inverse_blocks_diff(base_blocks: &[MdBlock], diff: &MdBlocksDiff) -> MdBlocksDiff {
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_blocks.get(m.index) {
            let next_index = transform_block_index(m.index, &diff.removed, &diff.added);
            modified.push(MdBlockModified { index: next_index, diff: inverse_block_diff(Some(original), &m.diff) });
        }
    }
    let mut added = Vec::new();
    for &idx in &diff.removed {
        if let Some(original) = base_blocks.get(idx) {
            added.push(MdBlockAdded { index: idx, item: original.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    MdBlocksDiff { removed, modified, added }
}

fn inverse_block_diff(current: Option<&MdBlock>, diff: &MdBlockDiff) -> MdBlockDiff {
    let fallback = || MdBlockDiff::Replace { block: current.cloned().unwrap_or(MdBlock::Paragraph { inlines: Vec::new() }) };
    match diff {
        MdBlockDiff::Replace { .. } => fallback(),
        MdBlockDiff::Heading { level, inlines } => match current {
            Some(MdBlock::Heading { level: l, inlines: i }) => {
                MdBlockDiff::Heading { level: level.as_ref().map(|_| *l), inlines: inlines.as_ref().map(|_| i.clone()) }
            }
            Some(other) => MdBlockDiff::Replace { block: other.clone() },
            None => fallback(),
        },
        MdBlockDiff::Paragraph { inlines } => match current {
            Some(MdBlock::Paragraph { inlines: i }) => MdBlockDiff::Paragraph { inlines: inlines.as_ref().map(|_| i.clone()) },
            Some(other) => MdBlockDiff::Replace { block: other.clone() },
            None => fallback(),
        },
        MdBlockDiff::CodeBlock { info, literal } => match current {
            Some(MdBlock::CodeBlock { info: i, literal: l }) => {
                MdBlockDiff::CodeBlock { info: info.as_ref().map(|_| i.clone()), literal: literal.as_ref().map(|_| l.clone()) }
            }
            Some(other) => MdBlockDiff::Replace { block: other.clone() },
            None => fallback(),
        },
        MdBlockDiff::HtmlBlock { raw } => match current {
            Some(MdBlock::HtmlBlock { raw: r }) => MdBlockDiff::HtmlBlock { raw: raw.as_ref().map(|_| r.clone()) },
            Some(other) => MdBlockDiff::Replace { block: other.clone() },
            None => fallback(),
        },
        MdBlockDiff::ThematicBreak => MdBlockDiff::ThematicBreak,
        MdBlockDiff::BlockQuote { blocks } => match current {
            Some(MdBlock::BlockQuote { blocks: b }) => {
                MdBlockDiff::BlockQuote { blocks: blocks.as_ref().map(|bd| inverse_blocks_diff(b, bd)) }
            }
            Some(other) => MdBlockDiff::Replace { block: other.clone() },
            None => fallback(),
        },
        MdBlockDiff::List { ordered, start, tight, items } => match current {
            Some(MdBlock::List { ordered: o, start: s, tight: t, items: it }) => MdBlockDiff::List {
                ordered: ordered.as_ref().map(|_| *o),
                start: start.as_ref().map(|_| *s),
                tight: tight.as_ref().map(|_| *t),
                items: items.as_ref().map(|id| inverse_list_items_diff(it, id)),
            },
            Some(other) => MdBlockDiff::Replace { block: other.clone() },
            None => fallback(),
        },
    }
}

fn inverse_list_items_diff(base_items: &[Vec<MdBlock>], diff: &MdListItemsDiff) -> MdListItemsDiff {
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.get(m.index) {
            let next_index = transform_item_index(m.index, &diff.removed, &diff.added);
            modified.push(MdListItemModified { index: next_index, diff: inverse_blocks_diff(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for &idx in &diff.removed {
        if let Some(original) = base_items.get(idx) {
            added.push(MdListItemAdded { index: idx, item: original.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    MdListItemsDiff { removed, modified, added }
}

fn between_blocks(base: &[MdBlock], other: &[MdBlock]) -> Option<MdBlocksDiff> {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            if let Some(d) = between_block(&base[i], &other[i]) {
                modified.push(MdBlockModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (other.len()..base.len()).collect();
    let added: Vec<MdBlockAdded> = (min_len..other.len()).map(|i| MdBlockAdded { index: i, item: other[i].clone() }).collect();
    if modified.is_empty() && removed.is_empty() && added.is_empty() { None } else { Some(MdBlocksDiff { removed, modified, added }) }
}

fn between_block(base: &MdBlock, other: &MdBlock) -> Option<MdBlockDiff> {
    if base == other {
        return None;
    }
    match (base, other) {
        (MdBlock::Heading { level: bl, inlines: bi }, MdBlock::Heading { level: ol, inlines: oi }) => Some(MdBlockDiff::Heading {
            level: if bl != ol { Some(*ol) } else { None },
            inlines: if bi != oi { Some(oi.clone()) } else { None },
        }),
        (MdBlock::Paragraph { inlines: bi }, MdBlock::Paragraph { inlines: oi }) => {
            if bi == oi { None } else { Some(MdBlockDiff::Paragraph { inlines: Some(oi.clone()) }) }
        }
        (MdBlock::CodeBlock { info: bin, literal: bl }, MdBlock::CodeBlock { info: oin, literal: ol }) => Some(MdBlockDiff::CodeBlock {
            info: if bin != oin { Some(oin.clone()) } else { None },
            literal: if bl != ol { Some(ol.clone()) } else { None },
        }),
        (MdBlock::HtmlBlock { raw: br }, MdBlock::HtmlBlock { raw: or }) => {
            if br == or { None } else { Some(MdBlockDiff::HtmlBlock { raw: Some(or.clone()) }) }
        }
        (MdBlock::ThematicBreak, MdBlock::ThematicBreak) => None,
        (MdBlock::BlockQuote { blocks: bb }, MdBlock::BlockQuote { blocks: ob }) => {
            between_blocks(bb, ob).map(|bd| MdBlockDiff::BlockQuote { blocks: Some(bd) })
        }
        (
            MdBlock::List { ordered: bo, start: bs, tight: bt, items: bi },
            MdBlock::List { ordered: oo, start: os, tight: ot, items: oi },
        ) => {
            let ordered = if bo != oo { Some(*oo) } else { None };
            let start = if bs != os { Some(*os) } else { None };
            let tight = if bt != ot { Some(*ot) } else { None };
            let items = between_list_items(bi, oi);
            if ordered.is_none() && start.is_none() && tight.is_none() && items.is_none() {
                None
            } else {
                Some(MdBlockDiff::List { ordered, start, tight, items })
            }
        }
        _ => Some(MdBlockDiff::Replace { block: other.clone() }),
    }
}

/// 🧮️ Naive positional item diff, same recipe-specified rule as `between_blocks`/xml's
/// `between_children`: pairwise `0..min(len)`, base tail removed, other tail added.
fn between_list_items(base: &[Vec<MdBlock>], other: &[Vec<MdBlock>]) -> Option<MdListItemsDiff> {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            if let Some(d) = between_blocks(&base[i], &other[i]) {
                modified.push(MdListItemModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (other.len()..base.len()).collect();
    let added: Vec<MdListItemAdded> = (min_len..other.len()).map(|i| MdListItemAdded { index: i, item: other[i].clone() }).collect();
    if modified.is_empty() && removed.is_empty() && added.is_empty() { None } else { Some(MdListItemsDiff { removed, modified, added }) }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️Absorb
/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (base-free index-transport
/// over `d1`'s removed/added). `transform_block_index`/`simulate_block_mid_origins` mirror xml's
/// `transform_index`/`simulate_mid_origins` exactly, retyped for `MdBlockAdded`; the `*_item_*`
/// variants below are the same algorithm again for the `List.items` collection.
fn transform_block_index(idx: usize, removed: &[usize], added: &[MdBlockAdded]) -> usize {
    let removed_before = removed.iter().filter(|&&r| r < idx).count();
    let pos = idx - removed_before;
    let mut order: Vec<usize> = added.iter().map(|a| a.index).collect();
    order.sort_unstable();
    let mut shift = 0usize;
    for target in order {
        if target <= pos + shift {
            shift += 1;
        } else {
            break;
        }
    }
    pos + shift
}

enum BlockOrigin {
    Base(usize),
    Added(usize),
}

fn simulate_block_mid_origins(base_len: usize, removed: &[usize], added: &[MdBlockAdded]) -> Vec<BlockOrigin> {
    let mut mid: Vec<BlockOrigin> = (0..base_len).filter(|i| !removed.contains(i)).map(BlockOrigin::Base).collect();
    let mut order: Vec<(usize, usize)> = added.iter().enumerate().map(|(k, a)| (a.index, k)).collect();
    order.sort_by_key(|(idx, _)| *idx);
    for (idx, k) in order {
        let at = idx.min(mid.len());
        mid.insert(at, BlockOrigin::Added(k));
    }
    mid
}

fn absorb_block_diff(a: MdBlockDiff, b: MdBlockDiff) -> MdBlockDiff {
    match (a, b) {
        (_, MdBlockDiff::Replace { block }) => MdBlockDiff::Replace { block },
        (MdBlockDiff::Replace { block }, b) => MdBlockDiff::Replace { block: apply_block_diff(&block, &b) },
        (MdBlockDiff::Heading { level: la, inlines: ia }, MdBlockDiff::Heading { level: lb, inlines: ib }) => {
            MdBlockDiff::Heading { level: lb.or(la), inlines: ib.or(ia) }
        }
        (MdBlockDiff::Paragraph { inlines: ia }, MdBlockDiff::Paragraph { inlines: ib }) => MdBlockDiff::Paragraph { inlines: ib.or(ia) },
        (MdBlockDiff::CodeBlock { info: ia, literal: la }, MdBlockDiff::CodeBlock { info: ib, literal: lb }) => {
            MdBlockDiff::CodeBlock { info: ib.or(ia), literal: lb.or(la) }
        }
        (MdBlockDiff::HtmlBlock { raw: ra }, MdBlockDiff::HtmlBlock { raw: rb }) => MdBlockDiff::HtmlBlock { raw: rb.or(ra) },
        (MdBlockDiff::ThematicBreak, MdBlockDiff::ThematicBreak) => MdBlockDiff::ThematicBreak,
        (MdBlockDiff::BlockQuote { blocks: ba }, MdBlockDiff::BlockQuote { blocks: bb }) => MdBlockDiff::BlockQuote {
            blocks: match (ba, bb) {
                (None, x) => x,
                (x, None) => x,
                (Some(x), Some(y)) => Some(absorb_blocks_diff(x, y)),
            },
        },
        (
            MdBlockDiff::List { ordered: oa, start: sa, tight: ta, items: ia },
            MdBlockDiff::List { ordered: ob, start: sb, tight: tb, items: ib },
        ) => MdBlockDiff::List {
            ordered: ob.or(oa),
            start: sb.or(sa),
            tight: tb.or(ta),
            items: match (ia, ib) {
                (None, x) => x,
                (x, None) => x,
                (Some(x), Some(y)) => Some(absorb_list_items_diff(x, y)),
            },
        },
        // 🛡️ Kind-mismatched arms (should not arise outside a prior `Replace`, handled above) --
        // graceful fallback: the later diff wins rather than panicking.
        (_, b) => b,
    }
}

fn absorb_blocks_diff(d1: MdBlocksDiff, d2: MdBlocksDiff) -> MdBlocksDiff {
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map(|m| m + 1).unwrap_or(0);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map(|m| m + 1).unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_block_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified.clone();
    let mut working_added = d1.added.clone();
    let mut annihilated: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for &r2 in &d2.removed {
        match mid.get(r2) {
            Some(BlockOrigin::Base(bi)) => {
                if !removed.contains(bi) {
                    removed.push(*bi);
                }
                modified.retain(|m| &m.index != bi);
            }
            Some(BlockOrigin::Added(k)) => {
                annihilated.insert(*k);
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid.get(m2.index) {
            Some(BlockOrigin::Base(bi)) => {
                if removed.contains(bi) {
                    continue;
                }
                match modified.iter_mut().find(|m| &m.index == bi) {
                    Some(existing) => existing.diff = absorb_block_diff(existing.diff.clone(), m2.diff.clone()),
                    None => modified.push(MdBlockModified { index: *bi, diff: m2.diff.clone() }),
                }
            }
            Some(BlockOrigin::Added(k)) => {
                if annihilated.contains(k) {
                    continue;
                }
                if let Some(add) = working_added.get_mut(*k) {
                    add.item = apply_block_diff(&add.item, &m2.diff);
                }
            }
            None => {}
        }
    }

    let mut added = Vec::new();
    for (k, add) in working_added.into_iter().enumerate() {
        if annihilated.contains(&k) {
            continue;
        }
        let final_index = transform_block_index(add.index, &d2.removed, &d2.added);
        added.push(MdBlockAdded { index: final_index, item: add.item });
    }
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    MdBlocksDiff { removed, modified, added }
}

fn transform_item_index(idx: usize, removed: &[usize], added: &[MdListItemAdded]) -> usize {
    let removed_before = removed.iter().filter(|&&r| r < idx).count();
    let pos = idx - removed_before;
    let mut order: Vec<usize> = added.iter().map(|a| a.index).collect();
    order.sort_unstable();
    let mut shift = 0usize;
    for target in order {
        if target <= pos + shift {
            shift += 1;
        } else {
            break;
        }
    }
    pos + shift
}

enum ItemOrigin {
    Base(usize),
    Added(usize),
}

fn simulate_item_mid_origins(base_len: usize, removed: &[usize], added: &[MdListItemAdded]) -> Vec<ItemOrigin> {
    let mut mid: Vec<ItemOrigin> = (0..base_len).filter(|i| !removed.contains(i)).map(ItemOrigin::Base).collect();
    let mut order: Vec<(usize, usize)> = added.iter().enumerate().map(|(k, a)| (a.index, k)).collect();
    order.sort_by_key(|(idx, _)| *idx);
    for (idx, k) in order {
        let at = idx.min(mid.len());
        mid.insert(at, ItemOrigin::Added(k));
    }
    mid
}

fn absorb_list_items_diff(d1: MdListItemsDiff, d2: MdListItemsDiff) -> MdListItemsDiff {
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map(|m| m + 1).unwrap_or(0);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map(|m| m + 1).unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_item_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified.clone();
    let mut working_added = d1.added.clone();
    let mut annihilated: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for &r2 in &d2.removed {
        match mid.get(r2) {
            Some(ItemOrigin::Base(bi)) => {
                if !removed.contains(bi) {
                    removed.push(*bi);
                }
                modified.retain(|m| &m.index != bi);
            }
            Some(ItemOrigin::Added(k)) => {
                annihilated.insert(*k);
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid.get(m2.index) {
            Some(ItemOrigin::Base(bi)) => {
                if removed.contains(bi) {
                    continue;
                }
                match modified.iter_mut().find(|m| &m.index == bi) {
                    Some(existing) => existing.diff = absorb_blocks_diff(existing.diff.clone(), m2.diff.clone()),
                    None => modified.push(MdListItemModified { index: *bi, diff: m2.diff.clone() }),
                }
            }
            Some(ItemOrigin::Added(k)) => {
                if annihilated.contains(k) {
                    continue;
                }
                if let Some(add) = working_added.get_mut(*k) {
                    add.item = apply_blocks_diff(&add.item, &m2.diff);
                }
            }
            None => {}
        }
    }

    let mut added = Vec::new();
    for (k, add) in working_added.into_iter().enumerate() {
        if annihilated.contains(&k) {
            continue;
        }
        let final_index = transform_item_index(add.index, &d2.removed, &d2.added);
        added.push(MdListItemAdded { index: final_index, item: add.item });
    }
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    MdListItemsDiff { removed, modified, added }
}
//#endregion 🔖️Absorb

//#region 🔖️SetSnapshot
/// 🧩️ Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No `snapshot:
/// Option<MdSnapshot>` full-replace slot -- this IS `MdDiff::between`.
pub fn diff_set_snapshot(base: &MdSnapshot, next: &MdSnapshot) -> MdDiff {
    MdDiff::between(base, next)
}
//#endregion 🔖️SetSnapshot
