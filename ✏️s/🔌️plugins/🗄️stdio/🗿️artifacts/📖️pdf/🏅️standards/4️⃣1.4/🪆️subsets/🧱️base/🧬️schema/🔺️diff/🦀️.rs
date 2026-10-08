//! 🔺️ PdfDiff (1.4) — handcrafted sparse diff over the document's page tree.
//!
//! 📚️ **Why this is a collection triple and not three flat fields.** `PdfSnapshot` carries
//! `pages: Vec<PageDoc>` (a real PDF 1.4 document has a real page tree — see
//! `../📸️snapshot/🦀️.rs`), so its diff is the recipe's INDEX-KEYED triple:
//! `removed`/`modified`/`added`, `modified` carrying a sparse [`PdfPageDiff`] and `added` carrying
//! a whole [`PageDoc`]. That is what makes "delete page 12" a three-byte diff instead of a
//! whole-document replacement, and it is why there is still no `snapshot: Option<PdfSnapshot>`
//! full-replace slot on `PdfDiff`; document differences stay sparse and page-addressed.
//!
//! ✍️ **Why the codecs are handcrafted rather than derived.** `#[derive(dsl::DslDiff)]` generates a
//! printer whose exact token shape this module does not choose, and the three facet files next to
//! it (`📝️text/📖️.grammar.semio`, `💾️binary/📡️.protocol.semio`) have to state that
//! shape production for production. The sibling 1.7 standard hand-rolls its own `DiffCodec` for the
//! same reason and its grammar file is written from its own `format!` call sites; this one follows
//! it exactly, at 1.4's own much smaller field set (`W`=width, `H`=height, `X`=text).

use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{ApplyCapability, MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};

//#region 🔖️PageDiff
/// 📄️ Sparse per-field patch for one [`PageDoc`] — a WEAK entity per the recipe (a value struct,
/// never sub-diffed beyond its own flat fields). No tri-state field exists: `PageDoc` has no
/// optional field of its own.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_page_diff(page: PageDoc, diff: &PdfPageDiff) -> PageDoc {
    PageDoc { width: diff.width.unwrap_or(page.width), height: diff.height.unwrap_or(page.height), text: diff.text.clone().unwrap_or(page.text) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_page_diff_empty(diff: &PdfPageDiff) -> bool {
    diff == &PdfPageDiff::default()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_page_diff(base: PdfPageDiff, other: PdfPageDiff) -> PdfPageDiff {
    PdfPageDiff { width: other.width.or(base.width), height: other.height.or(base.height), text: other.text.or(base.text) }
}
/// ↩️ The negative patch for `diff` over `page`: every field the patch names is set back to the value `page` holds.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_page_diff(diff: &PdfPageDiff, page: &PageDoc) -> PdfPageDiff {
    PdfPageDiff { width: diff.width.map(|_| page.width), height: diff.height.map(|_| page.height), text: diff.text.as_ref().map(|_| page.text.clone()) }
}
//#endregion 🔖️PageDiff

//#region 🔖️PagesTriple
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageModified {
    pub index: usize,
    pub diff: PdfPageDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageAdded {
    pub index: usize,
    pub page: PageDoc,
}

/// 📦️ Index-keyed `pages` triple (positional — the recipe's "index usize" key kind).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPagesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfPageModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfPageAdded>,
}

impl PdfPagesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Apply semantics (normative): `removed`/`modified` indices refer to BASE state (removals
/// processed descending); `added` indices refer to FINAL state (ascending insert).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_pages_diff(diff: &PdfPagesDiff, base: &[PageDoc]) -> Vec<PageDoc> {
    let mut pages: Vec<PageDoc> = base.to_vec();
    for modified in &diff.modified {
        if let Some(slot) = pages.get_mut(modified.index) {
            *slot = apply_page_diff(std::mem::take(slot), &modified.diff);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable();
    removed_sorted.dedup();
    for index in removed_sorted.into_iter().rev() {
        if index < pages.len() {
            pages.remove(index);
        }
    }
    let mut added_sorted: Vec<&PdfPageAdded> = diff.added.iter().collect();
    added_sorted.sort_by_key(|added| added.index);
    for added in added_sorted {
        pages.insert(added.index.min(pages.len()), added.page.clone());
    }
    pages
}

/// ➕️ Index-transported absorb via symbolic position simulation — the recipe's canonical algorithm
/// (`Insert+Remove-before`, `Insert+Insert` at one index both surviving, `Add+SetField` patching
/// into the carried added payload), specialized to the flat [`PdfPageDiff`] because a page is a
/// weak entity with no nested collection of its own.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_pages_diff(first: &PdfPagesDiff, second: &PdfPagesDiff) -> PdfPagesDiff {
    enum Origin {
        Base(usize),
        FirstAdded(usize),
    }
    enum AfterSlot {
        Base { original: usize, diff: Option<PdfPageDiff> },
        FirstAdded { tag: usize, patch: Option<PdfPageDiff> },
        SecondAdded(PageDoc),
    }

    let max_referenced = first
        .removed
        .iter()
        .copied()
        .chain(first.modified.iter().map(|item| item.index))
        .chain(first.added.iter().map(|item| item.index))
        .chain(second.removed.iter().copied())
        .chain(second.modified.iter().map(|item| item.index))
        .chain(second.added.iter().map(|item| item.index))
        .max()
        .unwrap_or(0);
    let simulated = max_referenced + first.removed.len() + second.removed.len() + 64;

    let mut middle: Vec<Origin> = (0..simulated).map(Origin::Base).collect();
    let mut first_removed = first.removed.clone();
    first_removed.sort_unstable();
    first_removed.dedup();
    for index in first_removed.iter().rev() {
        if *index < middle.len() {
            middle.remove(*index);
        }
    }
    let mut first_added_order: Vec<usize> = (0..first.added.len()).collect();
    first_added_order.sort_by_key(|tag| first.added[*tag].index);
    for tag in first_added_order {
        let position = first.added[tag].index.min(middle.len());
        middle.insert(position, Origin::FirstAdded(tag));
    }
    let first_modified: HashMap<usize, PdfPageDiff> = first.modified.iter().map(|item| (item.index, item.diff.clone())).collect();

    let mut after: Vec<AfterSlot> = middle
        .iter()
        .map(|origin| match origin {
            Origin::Base(original) => AfterSlot::Base { original: *original, diff: first_modified.get(original).cloned() },
            Origin::FirstAdded(tag) => AfterSlot::FirstAdded { tag: *tag, patch: None },
        })
        .collect();

    let mut final_removed: Vec<usize> = first.removed.clone();
    let mut second_removed = second.removed.clone();
    second_removed.sort_unstable();
    second_removed.dedup();
    for index in second_removed.iter().rev() {
        if *index < after.len() {
            if let AfterSlot::Base { original, .. } = after.remove(*index) {
                final_removed.push(original);
            }
        }
    }
    for modified in &second.modified {
        if let Some(slot) = after.get_mut(modified.index) {
            let target = match slot {
                AfterSlot::Base { diff, .. } => Some(diff),
                AfterSlot::FirstAdded { patch, .. } => Some(patch),
                AfterSlot::SecondAdded(_) => None,
            };
            if let Some(target) = target {
                let combined = match target.take() {
                    Some(existing) => absorb_page_diff(existing, modified.diff.clone()),
                    None => modified.diff.clone(),
                };
                *target = (!is_page_diff_empty(&combined)).then_some(combined);
            }
        }
    }
    let mut second_added_order: Vec<usize> = (0..second.added.len()).collect();
    second_added_order.sort_by_key(|tag| second.added[*tag].index);
    for tag in second_added_order {
        let position = second.added[tag].index.min(after.len());
        after.insert(position, AfterSlot::SecondAdded(second.added[tag].page.clone()));
    }

    let mut modified = Vec::new();
    let mut added = Vec::new();
    for (position, slot) in after.into_iter().enumerate() {
        match slot {
            AfterSlot::Base { original, diff: Some(diff) } => modified.push(PdfPageModified { index: original, diff }),
            AfterSlot::Base { .. } => {}
            AfterSlot::FirstAdded { tag, patch } => {
                let page = match patch {
                    Some(patch) => apply_page_diff(first.added[tag].page.clone(), &patch),
                    None => first.added[tag].page.clone(),
                };
                added.push(PdfPageAdded { index: position, page });
            }
            AfterSlot::SecondAdded(page) => added.push(PdfPageAdded { index: position, page }),
        }
    }
    final_removed.sort_unstable();
    final_removed.dedup();
    PdfPagesDiff { removed: final_removed, modified, added }
}

/// 🛡️ The index triple's own well-formedness, checked before anything is applied: a removal or a
/// modification must name a page the base has, no index may repeat, and an addition must land
/// inside the collection the diff itself produces.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_pages_diff(diff: &PdfPagesDiff, base_len: usize) -> MutationApplyResult<()> {
    let at = |index: usize| vec!["pages".to_string(), index.to_string()];
    let mut removed = HashSet::new();
    for index in &diff.removed {
        if *index >= base_len {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed removal target does not exist").at(at(*index)));
        }
        if !removed.insert(*index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed removal target is repeated").at(at(*index)));
        }
    }
    let mut modified = HashSet::new();
    for item in &diff.modified {
        if item.index >= base_len {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed modification target does not exist").at(at(item.index)));
        }
        if removed.contains(&item.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "indexed modification targets a removed page").at(at(item.index)));
        }
        if !modified.insert(item.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed modification target is repeated").at(at(item.index)));
        }
    }
    let final_len = base_len - removed.len() + diff.added.len();
    let mut added = HashSet::new();
    for item in &diff.added {
        if item.index >= final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "indexed addition is outside the final collection").at(at(item.index)));
        }
        if !added.insert(item.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed addition occupies a repeated final position").at(at(item.index)));
        }
    }
    Ok(())
}
/// ↩️ The negative page triple over `base`: it removes what `diff` added, patches every modified page back through its own
/// negative patch at the position the page holds in the final state, and re-adds every removed base page at its base index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_pages_diff(diff: &PdfPagesDiff, base: &[PageDoc]) -> PdfPagesDiff {
    enum Origin {
        Base(usize),
        Added,
    }
    let mut layout: Vec<Origin> = (0..base.len()).map(Origin::Base).collect();
    let mut removed = diff.removed.clone();
    removed.sort_unstable();
    removed.dedup();
    for index in removed.iter().rev() {
        if *index < layout.len() {
            layout.remove(*index);
        }
    }
    let mut added_order: Vec<usize> = (0..diff.added.len()).collect();
    added_order.sort_by_key(|tag| diff.added[*tag].index);
    for tag in added_order {
        layout.insert(diff.added[tag].index.min(layout.len()), Origin::Added);
    }
    let mut inverse = PdfPagesDiff::default();
    for (position, origin) in layout.into_iter().enumerate() {
        match origin {
            Origin::Added => inverse.removed.push(position),
            Origin::Base(index) => {
                if let (Some(modified), Some(page)) = (diff.modified.iter().find(|item| item.index == index), base.get(index)) {
                    inverse.modified.push(PdfPageModified { index: position, diff: inverse_page_diff(&modified.diff, page) });
                }
            }
        }
    }
    for index in removed {
        if let Some(page) = base.get(index) {
            inverse.added.push(PdfPageAdded { index, page: page.clone() });
        }
    }
    inverse
}
//#endregion 🔖️PagesTriple

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.pdf` (1.4). `schema` is an identity field and is never diffed.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.diff")]
pub struct PdfDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<PdfPagesDiff>,
}

impl MutationDiff<PdfSnapshot> for PdfDiff {
    fn apply(&self, base: &PdfSnapshot, _capability: ApplyCapability) -> MutationApplyResult<PdfSnapshot> {
        let mut next = base.clone();
        if let Some(pages) = &self.pages {
            validate_pages_diff(pages, base.pages.len())?;
            next.pages = apply_pages_diff(pages, &base.pages);
        }
        Ok(next)
    }

    /// ➕️ Structural, total, base-free, sequential-coalesce (`## Absorb` contract) — the page
    /// triple composes through [`absorb_pages_diff`]'s index-transported simulation.
    fn absorb(&mut self, other: Self) {
        self.pages = match (self.pages.take(), other.pages) {
            (None, other) => other,
            (mine, None) => mine,
            (Some(mine), Some(other)) => {
                let combined = absorb_pages_diff(&mine, &other);
                (!combined.is_empty()).then_some(combined)
            }
        };
    }
}

impl DiffAlgebra<PdfSnapshot> for PdfDiff {
    /// 🔁️ Diff-level undo, built from `base`: the page triple that removes what this one added, restores what it modified
    /// and re-adds what it removed.
    fn inverse(&self, base: &PdfSnapshot) -> Self {
        PdfDiff { pages: self.pages.as_ref().map(|pages| inverse_pages_diff(pages, &base.pages)) }
    }

    fn is_empty(&self) -> bool {
        self.pages.is_none()
    }
}

//#endregion 🔖️Diff

//#region 🔖️TextCodec



















//#endregion 🔖️TextCodec

//#region 🔖️BinaryCodec









//#endregion 🔖️BinaryCodec

//#region 🔖️DiffCodec

//#endregion 🔖️DiffCodec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
