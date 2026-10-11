//! 🔺️ MdDiff — handcrafted recursive tree diff. `blocks` is an index-keyed recursive triple over
//! the top-level `MdBlock` sequence; `MdBlockDiff` is shaped like the `MdBlock` it targets, with
//! `List.items` and `BlockQuote.blocks` nesting their OWN index-keyed triples (`MdListItemsDiff`
//! reuses this same `MdBlocksDiff`/`MdBlockDiff` shape recursively for each item's content, since
//! a list item's content IS a `Vec<MdBlock>` -- identical to the top level and to a block quote's
//! content). `MdInline` is treated as a WEAK entity throughout (recipe: weak entities are
//! whole-value replaced) -- every `inlines`/`text` field below is `Option<Vec<MdInline>>` or
//! `Option<String>`, never sub-diffed. Same xml/svg tree-diff pattern (`.🧬semio/🦑️repo/🎫️tickets/
//! 🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/🧬️schema-design.md`,
//! xml's own diff module is the direct template this file follows arm-for-arm).

use crate::schema::snapshot::{MdBlock, MdInline};
use crate::MdSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.md`.
/// 🧪️ F6: `#[derive(dsl::)]` on this struct is structurally blocked for the SAME two
/// independent reasons `GifDiff`/`SvgDiff` hit (see `f6-recon-report.md` §3): (1) `MdBlockDiff` is
/// a genuine data-carrying enum reachable from `blocks: Option<MdBlocksDiff>` — `DslField` has no
/// impl for it (only `DslRecord`-derived structs and `DslScalar`-derived UNIT-only enums implement
/// `DslField`), same `E0277: the trait bound '...MdBlockDiff: DslField' is not satisfied` shape
/// `SvgNodeDiff` hit; (2) `MdBlockDiff::List.start: Option<Option<u32>>` and
/// `MdBlockDiff::CodeBlock.info: Option<Option<String>>` are tri-state `Option<Option<_>>` fields —
/// same `classify_field` single-peel blocker `GifFrameDiff` hit (no `impl<T: DslField> DslField for
/// Option<T>` exists anywhere in the `dsl` crate). `DiffCodec` is hand-rolled below
/// (`#region 🔖️HandcraftedDiffCodec`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md.diff")]
pub struct MdDiff {
    /// 🌳 `None` = top-level block sequence unchanged; `Some(diff)` = index-keyed recursive
    /// triple over it.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<MdBlocksDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️BlocksDiff
/// 🌳 Index-keyed, recursive block-sequence triple. `removed`/`modified` indices refer to BASE
/// state (descending removal order on apply); `added` indices refer to FINAL state (ascending
/// insert). Reused verbatim (same type) for `List.items[n]`'s content AND `BlockQuote.blocks` --
/// both are `Vec<MdBlock>`, exactly what this type diffs.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct MdBlocksDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<MdBlockModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<MdBlockAdded>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct MdBlockModified {
    pub index: usize,
    pub diff: MdBlockDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct MdBlockAdded {
    pub index: usize,
    pub item: MdBlock,
}

/// 🌳 Per-block diff, shaped like the `MdBlock` it targets. `Replace` is the fallback for a
/// block-KIND change (e.g. `Paragraph` -> `Heading`) -- every other variant assumes the target
/// keeps its kind.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum MdBlockDiff {
    Heading {
        #[value(default, skip_serializing_if = "Option::is_none")]
        level: Option<u8>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        inlines: Option<Vec<MdInline>>,
    },
    Paragraph {
        #[value(default, skip_serializing_if = "Option::is_none")]
        inlines: Option<Vec<MdInline>>,
    },
    List {
        #[value(default, skip_serializing_if = "Option::is_none")]
        ordered: Option<bool>,
        /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = start number cleared, `Some(Some(n))`
        /// = set.
        #[value(default, skip_serializing_if = "Option::is_none")]
        start: Option<Option<u32>>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        tight: Option<bool>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        items: Option<MdListItemsDiff>,
    },
    CodeBlock {
        /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = info string cleared, `Some(Some(s))`
        /// = set.
        #[value(default, skip_serializing_if = "Option::is_none")]
        info: Option<Option<String>>,
        #[value(default, skip_serializing_if = "Option::is_none")]
        literal: Option<String>,
    },
    BlockQuote {
        #[value(default, skip_serializing_if = "Option::is_none")]
        blocks: Option<MdBlocksDiff>,
    },
    /// 🔳 `ThematicBreak` carries no fields -- this variant only appears via a kind-preserving
    /// `between`/`apply` no-op (two `ThematicBreak`s are always structurally equal, so `between`
    /// never actually constructs it; included for match exhaustiveness/API symmetry with every
    /// other `MdBlock` kind).
    ThematicBreak,
    HtmlBlock {
        #[value(default, skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct MdListItemsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<MdListItemModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<MdListItemAdded>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct MdListItemModified {
    pub index: usize,
    pub diff: MdBlocksDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct MdListItemAdded {
    pub index: usize,
    pub item: Vec<MdBlock>,
}
//#endregion 🔖️ListItemsDiff

//#region 🔖️DiffAtPath
/// 🧭️ One descent step from a `Vec<MdBlock>` container down into a nested one.
/// `BlockQuote{index}` steps into the block-quote block at `index`'s own `blocks`;
/// `ListItem{index,item}` steps into the list block at `index`'s `items[item]`. Re-exported from
/// `crate::schema::mutations` for ergonomic access -- kept here, not in the
/// mutations module, so this module never needs to depend on it (mutations already depends on
/// diff).
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "step", rename_all = "camelCase")]
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
    fn apply(&self, base: &MdSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<MdSnapshot> {
        if let Some(blocks) = &self.blocks {
            validate_md_blocks(&base.blocks, blocks)?;
        }
        let mut next = base.clone();
        if let Some(bd) = &self.blocks {
            next.blocks = apply_blocks_diff(&next.blocks, bd);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.blocks = match (self.blocks.take(), other.blocks) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_blocks_rows(a, &b)),
        };
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_md_blocks(base: &[MdBlock], diff: &MdBlocksDiff) -> MutationApplyResult<()> {
    let mut removed = std::collections::HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() || !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "markdown block removal is missing or duplicated").at(["blocks", "removed"]));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() || !modified.insert(entry.index) || removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "markdown block modification is missing, duplicated, or removed").at(["blocks", "modified"]));
        }
        validate_md_block(&base[entry.index], &entry.diff)?;
    }
    let final_len = base.len().saturating_sub(diff.removed.len()).saturating_add(diff.added.len());
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len || !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "markdown block addition index is invalid or duplicated").at(["blocks", "added"]));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_md_list_items(base: &[Vec<MdBlock>], diff: &MdListItemsDiff) -> MutationApplyResult<()> {
    let mut removed = std::collections::HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() || !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "markdown list-item removal is missing or duplicated").at(["items", "removed"]));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() || !modified.insert(entry.index) || removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "markdown list-item modification is missing, duplicated, or removed").at(["items", "modified"]));
        }
        validate_md_blocks(&base[entry.index], &entry.diff)?;
    }
    let final_len = base.len().saturating_sub(diff.removed.len()).saturating_add(diff.added.len());
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len || !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "markdown list-item addition index is invalid or duplicated").at(["items", "added"]));
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
        (MdBlock::List { items, .. }, MdBlockDiff::List { items: Some(items_diff), .. }) => validate_md_list_items(items, items_diff),
        (MdBlock::List { .. }, MdBlockDiff::List { items: None, .. }) => Ok(()),
        (MdBlock::BlockQuote { blocks }, MdBlockDiff::BlockQuote { blocks: Some(blocks_diff) }) => validate_md_blocks(blocks, blocks_diff),
        (MdBlock::BlockQuote { .. }, MdBlockDiff::BlockQuote { blocks: None }) => Ok(()),
        _ => Err(MutationApplyError::new("mutation.apply.conflicting-target", "markdown block diff kind does not match its target").at(["blocks"])),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_blocks_diff(blocks: &[MdBlock], diff: &MdBlocksDiff) -> Vec<MdBlock> {
    let mut slots: Vec<Option<MdBlock>> = blocks.iter().cloned().map(Some).collect();
    for m in &diff.modified {
        if let Some(Some(b)) = slots.get(m.index) {
            let patched = apply_block_diff(b, &m.diff);
            slots[m.index] = Some(patched);
        }
    }
    let removed_sorted = semio_s_artifact_stdio_contract::ordered_unique_descending(&diff.removed);
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
                    Some(d) => apply_blocks_diff(b, d),
                    None => b.clone(),
                },
            },
            other => other.clone(),
        },
        MdBlockDiff::List { ordered, start, tight, items } => match block {
            MdBlock::List { ordered: o, start: s, tight: t, items: it } => MdBlock::List {
                ordered: ordered.unwrap_or(*o),
                start: (*start).unwrap_or(*s),
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
    let removed_sorted = semio_s_artifact_stdio_contract::ordered_unique_descending(&diff.removed);
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

    fn is_empty(&self) -> bool {
        self.blocks.is_none()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_blocks_diff(base_blocks: &[MdBlock], diff: &MdBlocksDiff) -> MdBlocksDiff {
    let mut removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    removed.sort_unstable();
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
    modified.sort_by_key(|m| m.index);
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
    let mut removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    removed.sort_unstable();
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
    modified.sort_by_key(|m| m.index);
    MdListItemsDiff { removed, modified, added }
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
fn absorb_block_rows(a: MdBlockDiff, b: MdBlockDiff) -> MdBlockDiff {
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
                (Some(x), Some(y)) => Some(absorb_blocks_rows(x, &y)),
            },
        },
        (MdBlockDiff::List { ordered: oa, start: sa, tight: ta, items: ia }, MdBlockDiff::List { ordered: ob, start: sb, tight: tb, items: ib }) => MdBlockDiff::List {
            ordered: ob.or(oa),
            start: sb.or(sa),
            tight: tb.or(ta),
            items: match (ia, ib) {
                (None, x) => x,
                (x, None) => x,
                (Some(x), Some(y)) => Some(absorb_list_items_rows(x, &y)),
            },
        },
        // 🛡️ Kind-mismatched arms (should not arise outside a prior `Replace`, handled above) --
        // graceful fallback: the later diff wins rather than panicking.
        (_, b) => b,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_blocks_rows(d1: MdBlocksDiff, d2: &MdBlocksDiff) -> MdBlocksDiff {
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map_or(0, |m| m + 1);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map_or(0, |m| m + 1);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_block_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified.clone();
    let mut working_added = d1.added;
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
                    Some(existing) => existing.diff = absorb_block_rows(existing.diff.clone(), m2.diff.clone()),
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
fn absorb_list_items_rows(d1: MdListItemsDiff, d2: &MdListItemsDiff) -> MdListItemsDiff {
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map_or(0, |m| m + 1);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map_or(0, |m| m + 1);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_item_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified.clone();
    let mut working_added = d1.added;
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
                    Some(existing) => existing.diff = absorb_blocks_rows(existing.diff.clone(), &m2.diff),
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
///
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
// 🚫️aaaaaaa️aaaaregion 🔖️Primitives

//#region 🔖️BinaryPrimitives
/// 🧪️aaaaa️aregion 🔖️BinaryPrimitives

//#region 🔖️InlineCodec
/// 🌳 aaagion 🔖️InlineBinaryCodec
/// 🧪️aaaregion 🔖️InlineBinaryCodec
//#endregion 🔖️InlineCodec

//#region 🔖️BlockCodec
/// 🧱 aaaaagion 🔖️BlockBinaryCodec
/// 🧪️aaaaaregion 🔖️BlockBinaryCodec
//#endregion 🔖️BlockCodec

//#region 🔖️DiffValueCodecs
/// 🌳 a� a� aregion 🔖️DiffValueCodecs

//#region 🔖️DiffValueBinaryCodecs
/// 🧪️a� a� aregion 🔖️DiffValueBinaryCodecs

//#region 🔖️TopLevel
// 🚫️aaprregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `MdDiff` values — the single source of truth reused by
/// `diff_codec_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️.rs`'s
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn demo_snapshot(blocks: Vec<MdBlock>) -> MdSnapshot {
    MdSnapshot { schema: crate::STDIO_MD_DOCUMENT_SCHEMA.into(), blocks }
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
/// its 8 variants, `Replace` incl.), every `MdInline` variant (via
/// `all_inline_kinds`), both tri-states (`List.start`, `CodeBlock.info`), and both
/// `MdBlocksDiff`/`MdListItemsDiff` triples at multiple nesting depths (top-level,
/// `BlockQuote.blocks`, `List.items`), all declared by hand.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<MdDiff> {
    let mut cases = vec![
        MdDiff::default(),
        MdDiff {
            blocks: Some(MdBlocksDiff {
                removed: vec![1],
                modified: vec![
                    MdBlockModified { index: 0, diff: MdBlockDiff::Heading { level: Some(2), inlines: Some(all_inline_kinds()) } },
                    MdBlockModified { index: 2, diff: MdBlockDiff::List { ordered: Some(true), start: Some(Some(3)), tight: Some(false), items: None } },
                    MdBlockModified { index: 3, diff: MdBlockDiff::CodeBlock { info: Some(None), literal: Some("x".into()) } },
                ],
                added: vec![MdBlockAdded { index: 1, item: MdBlock::HtmlBlock { raw: "<p/>".into() } }],
            }),
        },
    ];
    // 🍃 Manual case: `ThematicBreak` diff + `Replace` at a nested `BlockQuote` depth, proving the codec handles both.
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
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests
