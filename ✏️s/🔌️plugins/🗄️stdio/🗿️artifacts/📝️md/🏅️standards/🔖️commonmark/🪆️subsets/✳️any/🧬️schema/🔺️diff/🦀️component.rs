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
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.md`.
/// 🧪️ F6: `#[derive(dsl::DslDiff)]` on this struct is structurally blocked for the SAME two
/// independent reasons `GifDiff`/`SvgDiff` hit (see `f6-recon-report.md` §3): (1) `MdBlockDiff` is
/// a genuine data-carrying enum reachable from `blocks: Option<MdBlocksDiff>` — `DslField` has no
/// impl for it (only `DslRecord`-derived structs and `DslScalar`-derived UNIT-only enums implement
/// `DslField`), same `E0277: the trait bound '...MdBlockDiff: DslField' is not satisfied` shape
/// `SvgNodeDiff` hit; (2) `MdBlockDiff::List.start: Option<Option<u32>>` and
/// `MdBlockDiff::CodeBlock.info: Option<Option<String>>` are tri-state `Option<Option<_>>` fields —
/// same `classify_field` single-peel blocker `GifFrameDiff` hit (no `impl<T: DslField> DslField for
/// Option<T>` exists anywhere in the `dsl` crate). `DiffCodec` is hand-rolled below
/// (`#region 🔖️HandcraftedDiffCodec`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md.diff")]
pub struct MdDiff {
    /// 🌳 `None` = top-level block sequence unchanged; `Some(diff)` = index-keyed recursive
    /// triple over it.
    #[state(artifact)]
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
    Replace { block: MdBlock },
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_at_path(path: &[MdPathStep], index: usize, leaf: MdBlocksLeafDiff) -> MdDiff {
    let inner = match leaf {
        MdBlocksLeafDiff::Modified(diff) => MdBlocksDiff { removed: Vec::new(), modified: vec![MdBlockModified { index, diff }], added: Vec::new() },
        MdBlocksLeafDiff::Added(block) => MdBlocksDiff { removed: Vec::new(), modified: Vec::new(), added: vec![MdBlockAdded { index, item: block }] },
        MdBlocksLeafDiff::Removed => MdBlocksDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() },
    };
    MdDiff { blocks: Some(wrap_blocks_diff(path, inner)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_blocks_diff(path: &[MdPathStep], inner: MdBlocksDiff) -> MdBlocksDiff {
    let mut current = inner;
    for step in path.iter().rev() {
        current = match step {
            MdPathStep::BlockQuote { index } => MdBlocksDiff { removed: Vec::new(), added: Vec::new(), modified: vec![MdBlockModified { index: *index, diff: MdBlockDiff::BlockQuote { blocks: Some(current) } }] },
            MdPathStep::ListItem { index, item } => MdBlocksDiff {
                removed: Vec::new(),
                added: Vec::new(),
                modified: vec![MdBlockModified {
                    index: *index,
                    diff: MdBlockDiff::List { ordered: None, start: None, tight: None, items: Some(MdListItemsDiff { removed: Vec::new(), added: Vec::new(), modified: vec![MdListItemModified { index: *item, diff: current }] }) },
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    async fn apply(&self, base: &MdSnapshot) -> MutationApplyResult<MdSnapshot> {
        if let Some(blocks) = &self.blocks {
            validate_md_blocks(&base.blocks, blocks)?;
        }
        let mut next = base.clone();
        if let Some(bd) = &self.blocks {
            next.blocks = apply_blocks_diff(&next.blocks, bd);
        }
        Ok(next)
    }

    async fn absorb(&mut self, other: Self) {
        self.blocks = match (self.blocks.take(), other.blocks) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_blocks_diff(a, b)),
        };
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_md_blocks(base: &[MdBlock], diff: &MdBlocksDiff) -> MutationApplyResult<()> {
    let mut removed = std::collections::HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() || !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "markdown block removal is missing or duplicated").await.at(["blocks", "removed"]));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() || !modified.insert(entry.index) || removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "markdown block modification is missing, duplicated, or removed").await.at(["blocks", "modified"]));
        }
        Box::pin(validate_md_block(&base[entry.index], &entry.diff))?;
    }
    let final_len = base.len().saturating_sub(diff.removed.len()).saturating_add(diff.added.len());
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len || !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "markdown block addition index is invalid or duplicated").await.at(["blocks", "added"]));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_md_list_items(base: &[Vec<MdBlock>], diff: &MdListItemsDiff) -> MutationApplyResult<()> {
    let mut removed = std::collections::HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() || !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "markdown list-item removal is missing or duplicated").await.at(["items", "removed"]));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() || !modified.insert(entry.index) || removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "markdown list-item modification is missing, duplicated, or removed").await.at(["items", "modified"]));
        }
        Box::pin(validate_md_blocks(&base[entry.index], &entry.diff))?;
    }
    let final_len = base.len().saturating_sub(diff.removed.len()).saturating_add(diff.added.len());
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len || !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "markdown list-item addition index is invalid or duplicated").await.at(["items", "added"]));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_md_block(base: &MdBlock, diff: &MdBlockDiff) -> MutationApplyResult<()> {
    match (base, diff) {
        (_, MdBlockDiff::Replace { .. })
        | (MdBlock::Heading { .. }, MdBlockDiff::Heading { .. })
        | (MdBlock::Paragraph { .. }, MdBlockDiff::Paragraph { .. })
        | (MdBlock::CodeBlock { .. }, MdBlockDiff::CodeBlock { .. })
        | (MdBlock::HtmlBlock { .. }, MdBlockDiff::HtmlBlock { .. })
        | (MdBlock::ThematicBreak, MdBlockDiff::ThematicBreak) => Ok(()),
        (MdBlock::List { items, .. }, MdBlockDiff::List { items: Some(items_diff), .. }) => Box::pin(validate_md_list_items(items, items_diff)),
        (MdBlock::List { .. }, MdBlockDiff::List { items: None, .. }) => Ok(()),
        (MdBlock::BlockQuote { blocks }, MdBlockDiff::BlockQuote { blocks: Some(blocks_diff) }) => validate_md_blocks(blocks, blocks_diff),
        (MdBlock::BlockQuote { .. }, MdBlockDiff::BlockQuote { blocks: None }) => Ok(()),
        _ => Err(MutationApplyError::new("mutation.apply.conflicting-target", "markdown block diff kind does not match its target").await.at(["blocks"])),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_blocks_diff(blocks: &[MdBlock], diff: &MdBlocksDiff) -> Vec<MdBlock> {
    let mut slots: Vec<Option<MdBlock>> = blocks.iter().cloned().map(Some).collect();
    for m in &diff.modified {
        if let Some(Some(b)) = slots.get(m.index) {
            let patched = apply_block_diff(b, &m.diff);
            slots[m.index] = Some(Box::pin(patched));
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_block_diff(block: &MdBlock, diff: &MdBlockDiff) -> MdBlock {
    match diff {
        MdBlockDiff::Replace { block: replacement } => replacement.clone(),
        MdBlockDiff::Heading { level, inlines } => match block {
            MdBlock::Heading { level: l, inlines: i } => MdBlock::Heading { level: level.unwrap_or(*l), inlines: inlines.clone().unwrap_or_else(|| i.clone()) },
            other => other.clone(),
        },
        MdBlockDiff::Paragraph { inlines } => match block {
            MdBlock::Paragraph { inlines: i } => MdBlock::Paragraph { inlines: inlines.clone().unwrap_or_else(|| i.clone()) },
            other => other.clone(),
        },
        MdBlockDiff::CodeBlock { info, literal } => match block {
            MdBlock::CodeBlock { info: i, literal: l } => MdBlock::CodeBlock { info: info.clone().unwrap_or_else(|| i.clone()), literal: literal.clone().unwrap_or_else(|| l.clone()) },
            other => other.clone(),
        },
        MdBlockDiff::HtmlBlock { raw } => match block {
            MdBlock::HtmlBlock { raw: r } => MdBlock::HtmlBlock { raw: raw.clone().unwrap_or_else(|| r.clone()) },
            other => other.clone(),
        },
        MdBlockDiff::ThematicBreak => MdBlock::ThematicBreak,
        MdBlockDiff::BlockQuote { blocks } => match block {
            MdBlock::BlockQuote { blocks: b } => MdBlock::BlockQuote {
                blocks: match blocks {
                    Some(d) => Box::pin(apply_blocks_diff(b, d)),
                    None => b.clone(),
                },
            },
            other => other.clone(),
        },
        MdBlockDiff::List { ordered, start, tight, items } => match block {
            MdBlock::List { ordered: o, start: s, tight: t, items: it } => MdBlock::List {
                ordered: ordered.unwrap_or(*o),
                start: start.clone().unwrap_or(*s),
                tight: tight.unwrap_or(*t),
                items: match items {
                    Some(d) => apply_list_items_diff(it, d),
                    None => it.clone(),
                },
            },
            other => other.clone(),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    async fn inverse(&self, base: &MdSnapshot) -> Self {
        MdDiff { blocks: self.blocks.as_ref().map(|d| inverse_blocks_diff(&base.blocks, d)) }
    }

    async fn between(base: &MdSnapshot, other: &MdSnapshot) -> Self {
        MdDiff { blocks: between_blocks(&base.blocks, &other.blocks) }
    }

    async fn is_empty(&self) -> bool {
        self.blocks.is_none()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_block_diff(current: Option<&MdBlock>, diff: &MdBlockDiff) -> MdBlockDiff {
    let fallback = || MdBlockDiff::Replace { block: current.cloned().unwrap_or(MdBlock::Paragraph { inlines: Vec::new() }) };
    match diff {
        MdBlockDiff::Replace { .. } => fallback(),
        MdBlockDiff::Heading { level, inlines } => match current {
            Some(MdBlock::Heading { level: l, inlines: i }) => MdBlockDiff::Heading { level: level.as_ref().map(|_| *l), inlines: inlines.as_ref().map(|_| i.clone()) },
            Some(other) => MdBlockDiff::Replace { block: other.clone() },
            None => fallback(),
        },
        MdBlockDiff::Paragraph { inlines } => match current {
            Some(MdBlock::Paragraph { inlines: i }) => MdBlockDiff::Paragraph { inlines: inlines.as_ref().map(|_| i.clone()) },
            Some(other) => MdBlockDiff::Replace { block: other.clone() },
            None => fallback(),
        },
        MdBlockDiff::CodeBlock { info, literal } => match current {
            Some(MdBlock::CodeBlock { info: i, literal: l }) => MdBlockDiff::CodeBlock { info: info.as_ref().map(|_| i.clone()), literal: literal.as_ref().map(|_| l.clone()) },
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
            Some(MdBlock::BlockQuote { blocks: b }) => MdBlockDiff::BlockQuote { blocks: blocks.as_ref().map(|bd| inverse_blocks_diff(b, bd)) },
            Some(other) => MdBlockDiff::Replace { block: other.clone() },
            None => fallback(),
        },
        MdBlockDiff::List { ordered, start, tight, items } => match current {
            Some(MdBlock::List { ordered: o, start: s, tight: t, items: it }) => {
                MdBlockDiff::List { ordered: ordered.as_ref().map(|_| *o), start: start.as_ref().map(|_| *s), tight: tight.as_ref().map(|_| *t), items: items.as_ref().map(|id| inverse_list_items_diff(it, id)) }
            }
            Some(other) => MdBlockDiff::Replace { block: other.clone() },
            None => fallback(),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_blocks(base: &[MdBlock], other: &[MdBlock]) -> Option<MdBlocksDiff> {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            if let Some(d) = Box::pin(between_block(&base[i], &other[i])) {
                modified.push(MdBlockModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (other.len()..base.len()).collect();
    let added: Vec<MdBlockAdded> = (min_len..other.len()).map(|i| MdBlockAdded { index: i, item: other[i].clone() }).collect();
    if modified.is_empty() && removed.is_empty() && added.is_empty() {
        None
    } else {
        Some(MdBlocksDiff { removed, modified, added })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_block(base: &MdBlock, other: &MdBlock) -> Option<MdBlockDiff> {
    if base == other {
        return None;
    }
    match (base, other) {
        (MdBlock::Heading { level: bl, inlines: bi }, MdBlock::Heading { level: ol, inlines: oi }) => Some(MdBlockDiff::Heading { level: if bl != ol { Some(*ol) } else { None }, inlines: if bi != oi { Some(oi.clone()) } else { None } }),
        (MdBlock::Paragraph { inlines: bi }, MdBlock::Paragraph { inlines: oi }) => {
            if bi == oi {
                None
            } else {
                Some(MdBlockDiff::Paragraph { inlines: Some(oi.clone()) })
            }
        }
        (MdBlock::CodeBlock { info: bin, literal: bl }, MdBlock::CodeBlock { info: oin, literal: ol }) => {
            Some(MdBlockDiff::CodeBlock { info: if bin != oin { Some(oin.clone()) } else { None }, literal: if bl != ol { Some(ol.clone()) } else { None } })
        }
        (MdBlock::HtmlBlock { raw: br }, MdBlock::HtmlBlock { raw: or }) => {
            if br == or {
                None
            } else {
                Some(MdBlockDiff::HtmlBlock { raw: Some(or.clone()) })
            }
        }
        (MdBlock::ThematicBreak, MdBlock::ThematicBreak) => None,
        (MdBlock::BlockQuote { blocks: bb }, MdBlock::BlockQuote { blocks: ob }) => Box::pin(between_blocks(bb, ob)).map(|bd| MdBlockDiff::BlockQuote { blocks: Some(bd) }),
        (MdBlock::List { ordered: bo, start: bs, tight: bt, items: bi }, MdBlock::List { ordered: oo, start: os, tight: ot, items: oi }) => {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    if modified.is_empty() && removed.is_empty() && added.is_empty() {
        None
    } else {
        Some(MdListItemsDiff { removed, modified, added })
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️Absorb
/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (base-free index-transport
/// over `d1`'s removed/added). `transform_block_index`/`simulate_block_mid_origins` mirror xml's
/// `transform_index`/`simulate_mid_origins` exactly, retyped for `MdBlockAdded`; the `*_item_*`
/// variants below are the same algorithm again for the `List.items` collection.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_block_diff(a: MdBlockDiff, b: MdBlockDiff) -> MdBlockDiff {
    match (a, b) {
        (_, MdBlockDiff::Replace { block }) => MdBlockDiff::Replace { block },
        (MdBlockDiff::Replace { block }, b) => MdBlockDiff::Replace { block: apply_block_diff(&block, &b) },
        (MdBlockDiff::Heading { level: la, inlines: ia }, MdBlockDiff::Heading { level: lb, inlines: ib }) => MdBlockDiff::Heading { level: lb.or(la), inlines: ib.or(ia) },
        (MdBlockDiff::Paragraph { inlines: ia }, MdBlockDiff::Paragraph { inlines: ib }) => MdBlockDiff::Paragraph { inlines: ib.or(ia) },
        (MdBlockDiff::CodeBlock { info: ia, literal: la }, MdBlockDiff::CodeBlock { info: ib, literal: lb }) => MdBlockDiff::CodeBlock { info: ib.or(ia), literal: lb.or(la) },
        (MdBlockDiff::HtmlBlock { raw: ra }, MdBlockDiff::HtmlBlock { raw: rb }) => MdBlockDiff::HtmlBlock { raw: rb.or(ra) },
        (MdBlockDiff::ThematicBreak, MdBlockDiff::ThematicBreak) => MdBlockDiff::ThematicBreak,
        (MdBlockDiff::BlockQuote { blocks: ba }, MdBlockDiff::BlockQuote { blocks: bb }) => MdBlockDiff::BlockQuote {
            blocks: match (ba, bb) {
                (None, x) => x,
                (x, None) => x,
                (Some(x), Some(y)) => Some(Box::pin(absorb_blocks_diff(x, y))),
            },
        },
        (MdBlockDiff::List { ordered: oa, start: sa, tight: ta, items: ia }, MdBlockDiff::List { ordered: ob, start: sb, tight: tb, items: ib }) => MdBlockDiff::List {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
                    Some(existing) => existing.diff = Box::pin(absorb_block_diff(existing.diff.clone(), m2.diff.clone())),
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &MdSnapshot, next: &MdSnapshot) -> MdDiff {
    MdDiff::between(base, next)
}
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` for `MdDiff` (real blocker citations on `MdDiff`'s own
/// doc comment above). This is the artifact with the MOST interacting enum kinds of any F6
/// hand-roll (`MdInline`, `MdBlock`, `MdBlockDiff` are all data-carrying) — each gets its OWN
/// non-overlapping single-uppercase-letter tag range so a tag can never be ambiguous about which
/// enum it belongs to, even though (same as `SvgNodeDiff`/`XmlNode` reusing `E`/`T`) letters WOULD
/// be safe to reuse across enums since every grammar position's expected type is statically known
/// by the recursive-descent parser -- kept disjoint anyway per the recon's explicit ask for this
/// artifact:
///   - `MdInline` (9 variants, declaration order): `A`=Text `B`=Emphasis `C`=Strong `D`=Code
///     `E`=Link `F`=Image `G`=SoftBreak `H`=HardBreak `I`=HtmlInline.
///   - `MdBlock` (7 variants): `J`=Heading `K`=Paragraph `L`=List `M`=CodeBlock `N`=BlockQuote
///     `O`=ThematicBreak `P`=HtmlBlock.
///   - `MdBlockDiff` (8 variants, same names as `MdBlock` + the `Replace` fallback): `Q`=Heading
///     `R`=Paragraph `S`=List `T`=CodeBlock `U`=BlockQuote `V`=ThematicBreak `W`=HtmlBlock
///     `X`=Replace.
///   - `MdPathStep` (mutations-side, 2 variants): `Y`=BlockQuote `Z`=ListItem.
/// Same grammar style as `GifDiff`/`SvgDiff` (bracket-depth-aware split, hex for strings, `[0]`/
/// `[1,x]` for `Option<T>`, nested `encode_option`/`decode_option` calls for `Option<Option<T>>`
/// tri-states) — primitives duplicated per-file by design (no shared "hand-roll helpers" module
/// exists yet, see `SvgDiff`'s doc comment for the rationale); everything a value-codec needs is
/// marked `pub(crate)` so `MdMutation`'s hand-rolled `OpText`/`OpBinary` (same file family as
/// `SvgMutation` reusing `SvgDiff`'s primitives) can reuse it rather than duplicating a second time.
///
/// 🧵️ One structural device worth flagging explicitly (not needed by `SvgDiff`, which never embeds
/// a BARE triple -- `SvgChildrenDiff`/`SvgAttributesDiff` -- directly inside another comma-joined
/// entry, only ever through `encode_option` or a tag-prefixed enum, both of which already supply an
/// enclosing bracket): `MdListItemsDiff.modified`'s `diff: MdBlocksDiff` field is a BARE triple
/// (`"[removed];[modified];[added]"`, no tag, no enclosing bracket of its own) embedded directly
/// inside a `,`-joined entry list. Left unwrapped, its internal `;` would sit at bracket-depth 0
/// relative to the OUTER `MdListItemsDiff` triple's own `;`-separated sections, corrupting that
/// outer split. Fix: `enc_list_items_diff`'s `modified` entries wrap the nested triple in an EXTRA
/// bracket pair (`format!("{}:[{}]", index, enc_blocks_diff(diff))`, mirrored by
/// `strip_brackets` on decode) so the nested `;`/`,` stay at depth ≥1 throughout -- the same
/// bracket-depth invariant `encode_option`'s `"[1,{value}]"` wrapping already gives every OTHER
/// triple-in-triple embedding in this file for free.
//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bool(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bool: expected 0/1, got {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
//#endregion 🔖️Primitives

//#region 🔖️BinaryPrimitives
/// 🧪️ P2-FG1: real LEB128-varint-framed binary primitives (length-prefixed strings, a tag-byte
/// `Option<T>` wrapper) backing the upgraded `OpBinary` (`../../🧬️mutations/🦀️component.rs`) and
/// `DiffCodec` (below) frames — mirrors json's own `write_str_lp`/`read_str_lp` shape, reusing
/// `store::pack_rt::write_varint_u64`/`store::ByteReader` rather than reinventing varint encode/
/// decode. `pub(crate)` so the mutations sibling can reuse these rather than duplicating them a
/// second time in that file (same intra-artifact-reuse split the TEXT codec primitives above use).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bool_bin(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 1 } else { 0 });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bool_bin(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().await.map_err(|e| e.to_string())? != 0)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
    String::from_utf8(reader.read_bytes(len).await.map_err(|e| e.to_string())?.to_vec()).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_option_bin<T>(out: &mut Vec<u8>, opt: &Option<T>, enc: impl FnOnce(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(v) => {
            out.push(1);
            enc(v, out);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_option_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<T>, String> {
    match reader.read_u8().await.map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(dec(reader)?)),
        other => Err(format!("option binary: unknown tag {other}")),
    }
}
/// 🏳️ Tri-state `Option<Option<T>>` binary wrapper (`MdBlockDiff::List.start`/`CodeBlock.info`) —
/// `0`=unchanged (`None`), `1`=cleared (`Some(None)`), `2`=set (`Some(Some(v))`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_tristate_bin<T>(out: &mut Vec<u8>, opt: &Option<Option<T>>, enc: impl FnOnce(&T, &mut Vec<u8>)) {
    match opt {
        None => out.push(0),
        Some(None) => out.push(1),
        Some(Some(v)) => {
            out.push(2);
            enc(v, out);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_tristate_bin<T>(reader: &mut store::ByteReader<'_>, dec: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<Option<Option<T>>, String> {
    match reader.read_u8().await.map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(None)),
        2 => Ok(Some(Some(dec(reader)?))),
        other => Err(format!("tristate binary: unknown tag {other}")),
    }
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️InlineCodec
/// 🌳 `MdInline`, tag range A-I (see file doc comment) — order matches its declaration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_inline(n: &MdInline) -> String {
    match n {
        MdInline::Text { text } => format!("A[{}]", enc_str(text)),
        MdInline::Emphasis { inlines } => format!("B[{}]", enc_inline_list(inlines)),
        MdInline::Strong { inlines } => format!("C[{}]", enc_inline_list(inlines)),
        MdInline::Code { literal } => format!("D[{}]", enc_str(literal)),
        MdInline::Link { text, url, title } => {
            format!("E[{},{},{}]", enc_inline_list(text), enc_str(url), encode_option(title, |v| enc_str(v)))
        }
        MdInline::Image { alt, url, title } => {
            format!("F[{},{},{}]", enc_str(alt), enc_str(url), encode_option(title, |v| enc_str(v)))
        }
        MdInline::SoftBreak => "G[]".to_string(),
        MdInline::HardBreak => "H[]".to_string(),
        MdInline::HtmlInline { raw } => format!("I[{}]", enc_str(raw)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_inline(s: &str) -> Result<MdInline, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "A" => Ok(MdInline::Text { text: dec_str(inner)? }),
        "B" => Ok(MdInline::Emphasis { inlines: dec_inline_list(inner)? }),
        "C" => Ok(MdInline::Strong { inlines: dec_inline_list(inner)? }),
        "D" => Ok(MdInline::Code { literal: dec_str(inner)? }),
        "E" => {
            let parts = split_top_level(inner, ',');
            let [text, url, title] = parts.as_slice() else { return Err(format!("inline link: expected 3 fields, got {}", parts.len())) };
            Ok(MdInline::Link { text: dec_inline_list(text)?, url: dec_str(url)?, title: decode_option(title, dec_str)? })
        }
        "F" => {
            let parts = split_top_level(inner, ',');
            let [alt, url, title] = parts.as_slice() else { return Err(format!("inline image: expected 3 fields, got {}", parts.len())) };
            Ok(MdInline::Image { alt: dec_str(alt)?, url: dec_str(url)?, title: decode_option(title, dec_str)? })
        }
        "G" => Ok(MdInline::SoftBreak),
        "H" => Ok(MdInline::HardBreak),
        "I" => Ok(MdInline::HtmlInline { raw: dec_str(inner)? }),
        other => Err(format!("inline: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_inline_list(list: &[MdInline]) -> String {
    format!("[{}]", list.iter().map(enc_inline).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_inline_list(s: &str) -> Result<Vec<MdInline>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_inline).collect()
}

//#region 🔖️InlineBinaryCodec
/// 🧪️ P2-FG1: real recursive binary twin of [`enc_inline`]/[`dec_inline`] above — same 0-8
/// ordinal order as the text codec's `A`-`I` tag range, backing the upgraded `OpBinary`/`DiffCodec`
/// frames (`../../🧬️mutations/🦀️component.rs`, `#region 🔖️TopLevel` below).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_inline_bin(n: &MdInline, out: &mut Vec<u8>) {
    match n {
        MdInline::Text { text } => {
            out.push(0);
            write_str_bin(out, text);
        }
        MdInline::Emphasis { inlines } => {
            out.push(1);
            enc_inline_list_bin(inlines, out);
        }
        MdInline::Strong { inlines } => {
            out.push(2);
            enc_inline_list_bin(inlines, out);
        }
        MdInline::Code { literal } => {
            out.push(3);
            write_str_bin(out, literal);
        }
        MdInline::Link { text, url, title } => {
            out.push(4);
            enc_inline_list_bin(text, out);
            write_str_bin(out, url);
            write_option_bin(out, title, |v, o| write_str_bin(o, v));
        }
        MdInline::Image { alt, url, title } => {
            out.push(5);
            write_str_bin(out, alt);
            write_str_bin(out, url);
            write_option_bin(out, title, |v, o| write_str_bin(o, v));
        }
        MdInline::SoftBreak => out.push(6),
        MdInline::HardBreak => out.push(7),
        MdInline::HtmlInline { raw } => {
            out.push(8);
            write_str_bin(out, raw);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_inline_bin(reader: &mut store::ByteReader<'_>) -> Result<MdInline, String> {
    let tag = reader.read_u8().await.map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(MdInline::Text { text: read_str_bin(reader)? }),
        1 => Ok(MdInline::Emphasis { inlines: dec_inline_list_bin(reader)? }),
        2 => Ok(MdInline::Strong { inlines: dec_inline_list_bin(reader)? }),
        3 => Ok(MdInline::Code { literal: read_str_bin(reader)? }),
        4 => {
            let text = dec_inline_list_bin(reader)?;
            let url = read_str_bin(reader)?;
            let title = read_option_bin(reader, read_str_bin)?;
            Ok(MdInline::Link { text, url, title })
        }
        5 => {
            let alt = read_str_bin(reader)?;
            let url = read_str_bin(reader)?;
            let title = read_option_bin(reader, read_str_bin)?;
            Ok(MdInline::Image { alt, url, title })
        }
        6 => Ok(MdInline::SoftBreak),
        7 => Ok(MdInline::HardBreak),
        8 => Ok(MdInline::HtmlInline { raw: read_str_bin(reader)? }),
        other => Err(format!("inline binary: unknown tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_inline_list_bin(list: &[MdInline], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for n in list {
        enc_inline_bin(n, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_inline_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<MdInline>, String> {
    let count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_inline_bin(reader)).collect()
}
//#endregion 🔖️InlineBinaryCodec
//#endregion 🔖️InlineCodec

//#region 🔖️BlockCodec
/// 🧱 `MdBlock`, tag range J-P (see file doc comment) — order matches its declaration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block(b: &MdBlock) -> String {
    match b {
        MdBlock::Heading { level, inlines } => format!("J[{},{}]", level, enc_inline_list(inlines)),
        MdBlock::Paragraph { inlines } => format!("K[{}]", enc_inline_list(inlines)),
        MdBlock::List { ordered, start, tight, items } => format!("L[{},{},{},{}]", enc_bool(*ordered), encode_option(start, |v| v.to_string()), enc_bool(*tight), enc_item_list(items),),
        MdBlock::CodeBlock { info, literal } => format!("M[{},{}]", encode_option(info, |v| enc_str(v)), enc_str(literal)),
        MdBlock::BlockQuote { blocks } => format!("N[{}]", enc_block_list(blocks)),
        MdBlock::ThematicBreak => "O[]".to_string(),
        MdBlock::HtmlBlock { raw } => format!("P[{}]", enc_str(raw)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block(s: &str) -> Result<MdBlock, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "J" => {
            let parts = split_top_level(inner, ',');
            let [level, inlines] = parts.as_slice() else { return Err(format!("heading: expected 2 fields, got {}", parts.len())) };
            Ok(MdBlock::Heading { level: level.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, inlines: dec_inline_list(inlines)? })
        }
        "K" => Ok(MdBlock::Paragraph { inlines: dec_inline_list(inner)? }),
        "L" => {
            let parts = split_top_level(inner, ',');
            let [ordered, start, tight, items] = parts.as_slice() else { return Err(format!("list: expected 4 fields, got {}", parts.len())) };
            Ok(MdBlock::List { ordered: dec_bool(ordered)?, start: decode_option(start, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, tight: dec_bool(tight)?, items: dec_item_list(items)? })
        }
        "M" => {
            let parts = split_top_level(inner, ',');
            let [info, literal] = parts.as_slice() else { return Err(format!("code block: expected 2 fields, got {}", parts.len())) };
            Ok(MdBlock::CodeBlock { info: decode_option(info, dec_str)?, literal: dec_str(literal)? })
        }
        "N" => Ok(MdBlock::BlockQuote { blocks: dec_block_list(inner)? }),
        "O" => Ok(MdBlock::ThematicBreak),
        "P" => Ok(MdBlock::HtmlBlock { raw: dec_str(inner)? }),
        other => Err(format!("block: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_list(list: &[MdBlock]) -> String {
    format!("[{}]", list.iter().map(enc_block).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_list(s: &str) -> Result<Vec<MdBlock>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_block).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_item_list(items: &[Vec<MdBlock>]) -> String {
    format!("[{}]", items.iter().map(|item| enc_block_list(item)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_item_list(s: &str) -> Result<Vec<Vec<MdBlock>>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_block_list).collect()
}

//#region 🔖️BlockBinaryCodec
/// 🧪️ P2-FG1: real recursive binary twin of [`enc_block`]/[`dec_block`] above — same 0-6 ordinal
/// order as the text codec's `J`-`P` tag range.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_bin(b: &MdBlock, out: &mut Vec<u8>) {
    match b {
        MdBlock::Heading { level, inlines } => {
            out.push(0);
            out.push(*level);
            enc_inline_list_bin(inlines, out);
        }
        MdBlock::Paragraph { inlines } => {
            out.push(1);
            enc_inline_list_bin(inlines, out);
        }
        MdBlock::List { ordered, start, tight, items } => {
            out.push(2);
            write_bool_bin(out, *ordered);
            write_option_bin(out, start, |v, o| { store::pack_rt::write_varint_u64(o, *v as u64); });
            write_bool_bin(out, *tight);
            enc_item_list_bin(items, out);
        }
        MdBlock::CodeBlock { info, literal } => {
            out.push(3);
            write_option_bin(out, info, |v, o| write_str_bin(o, v));
            write_str_bin(out, literal);
        }
        MdBlock::BlockQuote { blocks } => {
            out.push(4);
            enc_block_list_bin(blocks, out);
        }
        MdBlock::ThematicBreak => out.push(5),
        MdBlock::HtmlBlock { raw } => {
            out.push(6);
            write_str_bin(out, raw);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_bin(reader: &mut store::ByteReader<'_>) -> Result<MdBlock, String> {
    let tag = reader.read_u8().await.map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let level = reader.read_u8().await.map_err(|e| e.to_string())?;
            let inlines = dec_inline_list_bin(reader)?;
            Ok(MdBlock::Heading { level, inlines })
        }
        1 => Ok(MdBlock::Paragraph { inlines: dec_inline_list_bin(reader)? }),
        2 => {
            let ordered = read_bool_bin(reader)?;
            let start = read_option_bin(reader, |r| Ok(semio_framework_plugin::resolve_ready(r.read_varint_u64()).map_err(|e| e.to_string())? as u32))?;
            let tight = read_bool_bin(reader)?;
            let items = dec_item_list_bin(reader)?;
            Ok(MdBlock::List { ordered, start, tight, items })
        }
        3 => {
            let info = read_option_bin(reader, read_str_bin)?;
            let literal = read_str_bin(reader)?;
            Ok(MdBlock::CodeBlock { info, literal })
        }
        4 => Ok(MdBlock::BlockQuote { blocks: dec_block_list_bin(reader)? }),
        5 => Ok(MdBlock::ThematicBreak),
        6 => Ok(MdBlock::HtmlBlock { raw: read_str_bin(reader)? }),
        other => Err(format!("block binary: unknown tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_list_bin(list: &[MdBlock], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for b in list {
        enc_block_bin(b, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<MdBlock>, String> {
    let count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_block_bin(reader)).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_item_list_bin(items: &[Vec<MdBlock>], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for item in items {
        enc_block_list_bin(item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_item_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<Vec<MdBlock>>, String> {
    let count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    (0..count).map(|_| dec_block_list_bin(reader)).collect()
}
//#endregion 🔖️BlockBinaryCodec
//#endregion 🔖️BlockCodec

//#region 🔖️DiffValueCodecs
/// 🌳 `MdBlockDiff`, tag range Q-X (see file doc comment) — order matches its declaration, `X` =
/// `Replace` (the kind-change fallback, mirrors `SvgNodeDiff::Replace`'s `R`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_diff(d: &MdBlockDiff) -> String {
    match d {
        MdBlockDiff::Heading { level, inlines } => {
            format!("Q[{},{}]", encode_option(level, |v| v.to_string()), encode_option(inlines, |v| enc_inline_list(v)))
        }
        MdBlockDiff::Paragraph { inlines } => format!("R[{}]", encode_option(inlines, |v| enc_inline_list(v))),
        MdBlockDiff::List { ordered, start, tight, items } => format!(
            "S[{},{},{},{}]",
            encode_option(ordered, |v| enc_bool(*v).to_string()),
            encode_option(start, |v| encode_option(v, |x| x.to_string())),
            encode_option(tight, |v| enc_bool(*v).to_string()),
            encode_option(items, |v| enc_list_items_diff(v)),
        ),
        MdBlockDiff::CodeBlock { info, literal } => format!("T[{},{}]", encode_option(info, |v| encode_option(v, |x| enc_str(x))), encode_option(literal, |v| enc_str(v)),),
        MdBlockDiff::BlockQuote { blocks } => format!("U[{}]", encode_option(blocks, |v| enc_blocks_diff(v))),
        MdBlockDiff::ThematicBreak => "V[]".to_string(),
        MdBlockDiff::HtmlBlock { raw } => format!("W[{}]", encode_option(raw, |v| enc_str(v))),
        MdBlockDiff::Replace { block } => format!("X[{}]", enc_block(block)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_diff(s: &str) -> Result<MdBlockDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "Q" => {
            let parts = split_top_level(inner, ',');
            let [level, inlines] = parts.as_slice() else { return Err(format!("heading diff: expected 2 fields, got {}", parts.len())) };
            Ok(MdBlockDiff::Heading { level: decode_option(level, |v| v.parse().map_err(|e: std::num::ParseIntError| e.to_string()))?, inlines: decode_option(inlines, dec_inline_list)? })
        }
        "R" => Ok(MdBlockDiff::Paragraph { inlines: decode_option(inner, dec_inline_list)? }),
        "S" => {
            let parts = split_top_level(inner, ',');
            let [ordered, start, tight, items] = parts.as_slice() else { return Err(format!("list diff: expected 4 fields, got {}", parts.len())) };
            Ok(MdBlockDiff::List {
                ordered: decode_option(ordered, dec_bool)?,
                start: decode_option(start, |v| decode_option(v, |x| x.parse().map_err(|e: std::num::ParseIntError| e.to_string())))?,
                tight: decode_option(tight, dec_bool)?,
                items: decode_option(items, dec_list_items_diff)?,
            })
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [info, literal] = parts.as_slice() else { return Err(format!("code block diff: expected 2 fields, got {}", parts.len())) };
            Ok(MdBlockDiff::CodeBlock { info: decode_option(info, |v| decode_option(v, dec_str))?, literal: decode_option(literal, dec_str)? })
        }
        "U" => Ok(MdBlockDiff::BlockQuote { blocks: decode_option(inner, dec_blocks_diff)? }),
        "V" => Ok(MdBlockDiff::ThematicBreak),
        "W" => Ok(MdBlockDiff::HtmlBlock { raw: decode_option(inner, dec_str)? }),
        "X" => Ok(MdBlockDiff::Replace { block: dec_block(inner)? }),
        other => Err(format!("block diff: unknown tag {other:?}")),
    }
}

/// 🌳 `MdBlocksDiff` (BARE triple, no tag) — reused verbatim by `MdDiff.blocks`, `BlockQuote.blocks`,
/// and (via `MdListItemsDiff`) a `List` item's own content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_blocks_diff(d: &MdBlocksDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_block_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_block(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_blocks_diff(body: &str) -> Result<MdBlocksDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("blocks diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("block modified: bad entry {entry:?}"))?;
            Ok(MdBlockModified { index: parse_usize(idx)?, diff: dec_block_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("block added: bad entry {entry:?}"))?;
            Ok(MdBlockAdded { index: parse_usize(idx)?, item: dec_block(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(MdBlocksDiff { removed, modified, added })
}

/// 🌳 `MdListItemsDiff` (BARE triple, no tag) over a `List`'s `items: Vec<Vec<MdBlock>>`.
/// `modified` entries wrap their nested `MdBlocksDiff` in an EXTRA bracket pair (`{}:[{}]`, not
/// `{}:{}`) — see the region doc comment's "one structural device worth flagging" note for why a
/// bare triple embedded directly (not via `encode_option` or a tag-prefixed enum) needs it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_list_items_diff(d: &MdListItemsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:[{}]", m.index, enc_blocks_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_block_list(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_list_items_diff(body: &str) -> Result<MdListItemsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("list items diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("list item modified: bad entry {entry:?}"))?;
            Ok(MdListItemModified { index: parse_usize(idx)?, diff: dec_blocks_diff(strip_brackets(rest)?)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("list item added: bad entry {entry:?}"))?;
            Ok(MdListItemAdded { index: parse_usize(idx)?, item: dec_block_list(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(MdListItemsDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️ P2-FG1: real recursive binary twin of [`enc_block_diff`]/[`dec_block_diff`] — same 0-7
/// ordinal order as the text codec's `Q`-`X` tag range (`7`=`Replace`), backing the upgraded
/// `DiffCodec::encode_diff`/`decode_diff` below. `List`/`CodeBlock`'s tri-state fields use
/// [`write_tristate_bin`]/[`read_tristate_bin`]; every other `Option<T>` field uses the plain
/// [`write_option_bin`]/[`read_option_bin`] pair.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_block_diff_bin(d: &MdBlockDiff, out: &mut Vec<u8>) {
    match d {
        MdBlockDiff::Heading { level, inlines } => {
            out.push(0);
            write_option_bin(out, level, |v, o| o.push(*v));
            write_option_bin(out, inlines, |v, o| enc_inline_list_bin(v, o));
        }
        MdBlockDiff::Paragraph { inlines } => {
            out.push(1);
            write_option_bin(out, inlines, |v, o| enc_inline_list_bin(v, o));
        }
        MdBlockDiff::List { ordered, start, tight, items } => {
            out.push(2);
            write_option_bin(out, ordered, |v, o| write_bool_bin(o, *v));
            write_tristate_bin(out, start, |v, o| { store::pack_rt::write_varint_u64(o, *v as u64); });
            write_option_bin(out, tight, |v, o| write_bool_bin(o, *v));
            write_option_bin(out, items, |v, o| enc_list_items_diff_bin(v, o));
        }
        MdBlockDiff::CodeBlock { info, literal } => {
            out.push(3);
            write_tristate_bin(out, info, |v, o| write_str_bin(o, v));
            write_option_bin(out, literal, |v, o| write_str_bin(o, v));
        }
        MdBlockDiff::BlockQuote { blocks } => {
            out.push(4);
            write_option_bin(out, blocks, |v, o| enc_blocks_diff_bin(v, o));
        }
        MdBlockDiff::ThematicBreak => out.push(5),
        MdBlockDiff::HtmlBlock { raw } => {
            out.push(6);
            write_option_bin(out, raw, |v, o| write_str_bin(o, v));
        }
        MdBlockDiff::Replace { block } => {
            out.push(7);
            enc_block_bin(block, out);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_block_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<MdBlockDiff, String> {
    let tag = reader.read_u8().await.map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let level = read_option_bin(reader, |r| semio_framework_plugin::resolve_ready(r.read_u8()).map_err(|e| e.to_string()))?;
            let inlines = read_option_bin(reader, dec_inline_list_bin)?;
            Ok(MdBlockDiff::Heading { level, inlines })
        }
        1 => Ok(MdBlockDiff::Paragraph { inlines: read_option_bin(reader, dec_inline_list_bin)? }),
        2 => {
            let ordered = read_option_bin(reader, read_bool_bin)?;
            let start = read_tristate_bin(reader, |r| Ok(semio_framework_plugin::resolve_ready(r.read_varint_u64()).map_err(|e| e.to_string())? as u32))?;
            let tight = read_option_bin(reader, read_bool_bin)?;
            let items = read_option_bin(reader, dec_list_items_diff_bin)?;
            Ok(MdBlockDiff::List { ordered, start, tight, items })
        }
        3 => {
            let info = read_tristate_bin(reader, read_str_bin)?;
            let literal = read_option_bin(reader, read_str_bin)?;
            Ok(MdBlockDiff::CodeBlock { info, literal })
        }
        4 => Ok(MdBlockDiff::BlockQuote { blocks: read_option_bin(reader, dec_blocks_diff_bin)? }),
        5 => Ok(MdBlockDiff::ThematicBreak),
        6 => Ok(MdBlockDiff::HtmlBlock { raw: read_option_bin(reader, read_str_bin)? }),
        7 => Ok(MdBlockDiff::Replace { block: dec_block_bin(reader)? }),
        other => Err(format!("block diff binary: unknown tag {other}")),
    }
}

/// 🌳 `MdBlocksDiff` binary twin of [`enc_blocks_diff`]/[`dec_blocks_diff`] — three varint-counted,
/// recursively-encoded lists (removed/modified/added), genuinely structured binary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_blocks_diff_bin(d: &MdBlocksDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for idx in &d.removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for entry in &d.modified {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_block_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for entry in &d.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_block_bin(&entry.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_blocks_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<MdBlocksDiff, String> {
    let removed_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
        let diff = dec_block_diff_bin(reader)?;
        modified.push(MdBlockModified { index, diff });
    }
    let added_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
        let item = dec_block_bin(reader)?;
        added.push(MdBlockAdded { index, item });
    }
    Ok(MdBlocksDiff { removed, modified, added })
}

/// 🌳 `MdListItemsDiff` binary twin of [`enc_list_items_diff`]/[`dec_list_items_diff`] — same
/// 3-part shape, `modified.diff` a recursive `MdBlocksDiff`, `added.item` a `Vec<MdBlock>`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list_items_diff_bin(d: &MdListItemsDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, d.removed.len() as u64);
    for idx in &d.removed {
        store::pack_rt::write_varint_u64(out, *idx as u64);
    }
    store::pack_rt::write_varint_u64(out, d.modified.len() as u64);
    for entry in &d.modified {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_blocks_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, d.added.len() as u64);
    for entry in &d.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_block_list_bin(&entry.item, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list_items_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<MdListItemsDiff, String> {
    let removed_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
        let diff = dec_blocks_diff_bin(reader)?;
        modified.push(MdListItemModified { index, diff });
    }
    let added_count = reader.read_varint_u64().await.map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().await.map_err(|e| e.to_string())? as usize;
        let item = dec_block_list_bin(reader)?;
        added.push(MdListItemAdded { index, item });
    }
    Ok(MdListItemsDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_md_diff(d: &MdDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.blocks {
        tokens.push(format!("blocks={}", enc_blocks_diff(v)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_md_diff(line: &str) -> Result<MdDiff, String> {
    let mut d = MdDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("blocks=") {
            d.blocks = Some(dec_blocks_diff(rest)?);
        } else {
            return Err(format!("md diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for MdDiff {
    async fn print_diff(&self) -> String {
        print_md_diff(self)
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_md_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ P2-FG1: REAL binary frame (`format u8 | has_value u8 | blocks-diff payload`), matching
    /// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
    /// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100% of stdio's
    /// `DiffCodec` impls were still on that shortcut per the P2-W0 census).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, if self.blocks.is_some() { 1 } else { 0 }];
        if let Some(blocks) = &self.blocks {
            enc_blocks_diff_bin(blocks, &mut out);
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes).await;
        let _format = reader.read_u8().await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: e.to_string() })?;
        let has_value = reader.read_u8().await.map_err(|e| protocol::ProtocolError::Malformed { what: "diff has_value", offset: 1, detail: e.to_string() })?;
        let blocks = if has_value != 0 { Some(dec_blocks_diff_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff blocks", offset: semio_framework_plugin::resolve_ready(reader.position()) as u64, detail: e })?) } else { None };
        Ok(MdDiff { blocks })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `MdDiff` values — the single source of truth reused by
/// `diff_codec_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️component.rs`'s
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn demo_snapshot(blocks: Vec<MdBlock>) -> MdSnapshot {
    MdSnapshot { schema: crate::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA.into(), blocks }
}

/// 🌈 One instance of every `MdInline` variant (both `Option<title>` branches for
/// `Link`/`Image`), with `Emphasis`/`Strong` nesting another variant inside themselves so the
/// recursive `enc_inline_list`/`dec_inline_list` path gets exercised too.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn all_inline_kinds() -> Vec<MdInline> {
    vec![
        MdInline::Text { text: "hi".into() },
        MdInline::Emphasis { inlines: vec![MdInline::Text { text: "em".into() }] },
        MdInline::Strong { inlines: vec![MdInline::Code { literal: "x=1".into() }] },
        MdInline::Code { literal: "code".into() },
        MdInline::Link { text: vec![MdInline::Text { text: "go".into() }], url: "http://a".into(), title: Some("t".into()) },
        MdInline::Link { text: vec![MdInline::Text { text: "go2".into() }], url: "http://b".into(), title: None },
        MdInline::Image { alt: "pic".into(), url: "http://img".into(), title: Some("cap".into()) },
        MdInline::Image { alt: "pic2".into(), url: "http://img2".into(), title: None },
        MdInline::SoftBreak,
        MdInline::HardBreak,
        MdInline::HtmlInline { raw: "<br/>".into() },
    ]
}

/// 🌱 `md_a`/`md_b`: differ across every `MdBlockDiff` kind (`Heading`/`Paragraph`/`List`/
/// `CodeBlock`/`BlockQuote`/`HtmlBlock` via matched-kind field changes, plus one same-index
/// kind-CHANGE pair -- `HtmlBlock` -> `Heading` -- for `Replace`), both tri-states going
/// `Some(x) -> Some(None)` (`List.start`, `CodeBlock.info`), and an asymmetric length (9 vs 8
/// top-level blocks, 2 vs 3 `List` items) so `between(a,b)`/`between(b,a)` together exercise
/// `removed` AND `added` on BOTH `MdBlocksDiff` (top-level tail, `BlockQuote.blocks`) and
/// `MdListItemsDiff` (`List.items`) -- the same dual-direction trick `SvgMutation`'s
/// `sweep_a`/`sweep_b` fixtures use, since the recipe's naive positional `between` can only ever
/// show one of {removed-tail, added-tail} per single call. `md_a[6]`/`md_b[6]` are IDENTICAL
/// (proves an unchanged block correctly produces no diff entry at all).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_a() -> Vec<MdBlock> {
    vec![
        MdBlock::Heading { level: 1, inlines: vec![MdInline::Text { text: "Intro".into() }] },
        MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "para one".into() }, MdInline::SoftBreak] },
        MdBlock::List {
            ordered: false,
            start: Some(3),
            tight: true,
            items: vec![vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item-a1".into() }] }], vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item-a2".into() }] }]],
        },
        MdBlock::CodeBlock { info: Some("rust".into()), literal: "fn a(){}".into() },
        MdBlock::BlockQuote { blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "quoted-a".into() }] }] },
        MdBlock::HtmlBlock { raw: "<div>a</div>".into() },
        MdBlock::Paragraph { inlines: vec![MdInline::Strong { inlines: vec![MdInline::Text { text: "unchanged".into() }] }] },
        MdBlock::HtmlBlock { raw: "<span>willBecomeHeading</span>".into() },
        MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "tail-only-in-a".into() }] },
    ]
}
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn md_b() -> Vec<MdBlock> {
    vec![
        MdBlock::Heading { level: 2, inlines: all_inline_kinds() },
        MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "para one CHANGED".into() }] },
        MdBlock::List {
            ordered: true,
            start: None,
            tight: false,
            items: vec![
                vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item-b1".into() }] }],
                vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item-b2".into() }] }],
                vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item-b3-added".into() }] }],
            ],
        },
        MdBlock::CodeBlock { info: None, literal: "fn b(){}".into() },
        MdBlock::BlockQuote { blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "quoted-a".into() }] }, MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "quoted-b-added".into() }] }] },
        MdBlock::HtmlBlock { raw: "<div>b</div>".into() },
        MdBlock::Paragraph { inlines: vec![MdInline::Strong { inlines: vec![MdInline::Text { text: "unchanged".into() }] }] },
        MdBlock::Heading { level: 3, inlines: vec![MdInline::Text { text: "nowHeading".into() }] },
    ]
}

/// 🧪️ P2-FG1: representative `MdDiff` values — exercises the recursive `MdBlockDiff` enum (7 of
/// its 8 variants reachable via `between`, `Replace` incl.), every `MdInline` variant (via
/// `all_inline_kinds`), both tri-states (`List.start`, `CodeBlock.info`), and both
/// `MdBlocksDiff`/`MdListItemsDiff` triples at multiple nesting depths (top-level,
/// `BlockQuote.blocks`, `List.items`). `MdBlockDiff::ThematicBreak` is UNREACHABLE via `between`
/// (two `ThematicBreak`s are always structurally equal, per that variant's own doc comment) so it
/// gets one manually-constructed case here instead.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<MdDiff> {
    let a = demo_snapshot(md_a());
    let b = demo_snapshot(md_b());
    let empty = demo_snapshot(Vec::new());

    let mut cases = vec![MdDiff::default(), MdDiff::between(&a, &b), MdDiff::between(&b, &a), MdDiff::between(&a, &empty), MdDiff::between(&empty, &a)];
    // 🍃 Manual case: `ThematicBreak` diff (never produced by `between`) + `Replace` at a nested
    // `BlockQuote` depth, proving the codec handles both even off the `between` path.
    cases.push(MdDiff {
        blocks: Some(MdBlocksDiff {
            removed: Vec::new(),
            modified: vec![
                MdBlockModified { index: 0, diff: MdBlockDiff::ThematicBreak },
                MdBlockModified {
                    index: 1,
                    diff: MdBlockDiff::BlockQuote {
                        blocks: Some(MdBlocksDiff {
                            removed: vec![2, 0],
                            modified: vec![MdBlockModified { index: 1, diff: MdBlockDiff::Replace { block: MdBlock::ThematicBreak } }],
                            added: vec![MdBlockAdded { index: 0, item: MdBlock::HtmlBlock { raw: "<hr/>".into() } }],
                        }),
                    },
                },
            ],
            added: vec![MdBlockAdded { index: 2, item: MdBlock::List { ordered: true, start: Some(1), tight: false, items: vec![vec![MdBlock::ThematicBreak], Vec::new()] } }],
        }),
    });
    cases
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;
    use protocol::DiffCodec;

    /// 🧪️ F6/P2-FG1: `DiffCodec` round-trip laws over the hand-rolled `MdDiff` grammar — see
    /// `demo_diff_cases()`'s own doc comment for exactly what each case exercises.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.await.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = MdDiff::parse_diff(&printed).await.unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().await.unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = MdDiff::decode_diff(&encoded).await.unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
