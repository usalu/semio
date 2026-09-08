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

use crate::artifacts::pdf::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
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
fn apply_page_diff(page: &mut PageDoc, diff: &PdfPageDiff) {
    if let Some(value) = diff.width {
        page.width = value;
    }
    if let Some(value) = diff.height {
        page.height = value;
    }
    if let Some(value) = &diff.text {
        page.text = value.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_diff_between(a: &PageDoc, b: &PageDoc) -> PdfPageDiff {
    PdfPageDiff { width: (a.width != b.width).then_some(b.width), height: (a.height != b.height).then_some(b.height), text: (a.text != b.text).then(|| b.text.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_page_diff_empty(diff: &PdfPageDiff) -> bool {
    diff == &PdfPageDiff::default()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_page_diff(base: &mut PdfPageDiff, other: PdfPageDiff) {
    if other.width.is_some() {
        base.width = other.width;
    }
    if other.height.is_some() {
        base.height = other.height;
    }
    if other.text.is_some() {
        base.text = other.text;
    }
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
        if let Some(page) = pages.get_mut(modified.index) {
            apply_page_diff(page, &modified.diff);
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

/// 🧭️ `between` matching for index-keyed collections (recipe): pairwise `0..min(len)` as
/// `modified`, base tail as `removed`, other tail as `added`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pages_diff_between(a: &[PageDoc], b: &[PageDoc]) -> PdfPagesDiff {
    let common = a.len().min(b.len());
    let mut modified = Vec::new();
    for index in 0..common {
        let diff = page_diff_between(&a[index], &b[index]);
        if !is_page_diff_empty(&diff) {
            modified.push(PdfPageModified { index, diff });
        }
    }
    let removed: Vec<usize> = if a.len() > b.len() { (b.len()..a.len()).collect() } else { Vec::new() };
    let added: Vec<PdfPageAdded> = if b.len() > a.len() { (a.len()..b.len()).map(|index| PdfPageAdded { index, page: b[index].clone() }).collect() } else { Vec::new() };
    PdfPagesDiff { removed, modified, added }
}

/// ➕️ Index-transported absorb via symbolic position simulation — the recipe's canonical algorithm
/// (`Insert+Remove-before`, `Insert+Insert` at one index both surviving, `Add+SetField` patching
/// into the carried added payload), specialized to the flat [`PdfPageDiff`] because a page is a
/// weak entity with no nested collection of its own.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_pages_diff(first: PdfPagesDiff, second: PdfPagesDiff) -> PdfPagesDiff {
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
                    Some(mut existing) => {
                        absorb_page_diff(&mut existing, modified.diff.clone());
                        existing
                    }
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
                let mut page = first.added[tag].page.clone();
                if let Some(patch) = patch {
                    apply_page_diff(&mut page, &patch);
                }
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
    fn apply(&self, base: &PdfSnapshot) -> MutationApplyResult<PdfSnapshot> {
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
                let combined = absorb_pages_diff(mine, other);
                (!combined.is_empty()).then_some(combined)
            }
        };
    }
}

impl DiffAlgebra<PdfSnapshot> for PdfDiff {
    /// 🔁️ Diff-level undo, derived generically from `between` (correct by construction).
    fn inverse(&self, base: &PdfSnapshot) -> Self {
        let mid = self.apply(base).unwrap_or_else(|_| base.clone());
        Self::between(&mid, base)
    }

    fn between(base: &PdfSnapshot, other: &PdfSnapshot) -> Self {
        let pages = pages_diff_between(&base.pages, &other.pages);
        PdfDiff { pages: (!pages.is_empty()).then_some(pages) }
    }

    fn is_empty(&self) -> bool {
        self.pages.is_none()
    }
}

//#endregion 🔖️Diff

//#region 🔖️TextCodec
/// 🔤️ Hex, so a page's text can carry any byte (including the separators this grammar uses)
/// without an escape layer. Same primitive the sibling 1.7 diff codec uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(text: &str) -> String {
    text.bytes().map(|byte| format!("{byte:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(text: &str) -> Result<String, String> {
    if text.len() % 2 != 0 {
        return Err(format!("odd hex length: {text:?}"));
    }
    let bytes: Result<Vec<u8>, String> = (0..text.len()).step_by(2).map(|index| u8::from_str_radix(&text[index..index + 2], 16).map_err(|error| error.to_string())).collect();
    String::from_utf8(bytes?).map_err(|error| error.to_string())
}

/// 🧭️ Bracket-depth-aware split: a top-level `separator` inside nested brackets is never mistaken
/// for a field separator.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(text: &str, separator: char) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (index, character) in text.char_indices() {
        match character {
            '[' => depth += 1,
            ']' => depth -= 1,
            other if other == separator && depth == 0 => {
                out.push(&text[start..index]);
                start = index + other.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&text[start..]);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(text: &str) -> Result<&str, String> {
    text.strip_prefix('[').and_then(|inner| inner.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {text:?}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_usize(text: &str) -> Result<usize, String> {
    text.parse().map_err(|error: std::num::ParseIntError| error.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f64(text: &str) -> Result<f64, String> {
    text.parse().map_err(|error: std::num::ParseFloatError| error.to_string())
}

/// 📄️ A whole `PageDoc` literal: `[width,height,hex-text]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_page(page: &PageDoc) -> String {
    format!("[{},{},{}]", page.width, page.height, enc_str(&page.text))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_page(text: &str) -> Result<PageDoc, String> {
    let parts = split_top_level(strip_brackets(text)?, ',');
    let [width, height, body] = parts.as_slice() else { return Err(format!("page: expected 3 fields, got {}", parts.len())) };
    Ok(PageDoc { width: parse_f64(width)?, height: parse_f64(height)?, text: dec_str(body)? })
}

/// 🏷️ `PdfPageDiff`'s sparse fields as single-letter `tag:value` pairs: `W`=width, `H`=height,
/// `X`=text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_page_diff(diff: &PdfPageDiff) -> String {
    let mut parts = Vec::new();
    if let Some(value) = diff.width {
        parts.push(format!("W:{value}"));
    }
    if let Some(value) = diff.height {
        parts.push(format!("H:{value}"));
    }
    if let Some(value) = &diff.text {
        parts.push(format!("X:{}", enc_str(value)));
    }
    format!("[{}]", parts.join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_page_diff(text: &str) -> Result<PdfPageDiff, String> {
    let mut diff = PdfPageDiff::default();
    for entry in split_top_level(strip_brackets(text)?, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, value) = entry.split_once(':').ok_or_else(|| format!("page diff: bad entry {entry:?}"))?;
        match tag {
            "W" => diff.width = Some(parse_f64(value)?),
            "H" => diff.height = Some(parse_f64(value)?),
            "X" => diff.text = Some(dec_str(value)?),
            other => return Err(format!("page diff: unknown tag {other:?}")),
        }
    }
    Ok(diff)
}

/// 📦️ The triple: `[removed];[modified];[added]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_pages_diff(diff: &PdfPagesDiff) -> String {
    let removed = diff.removed.iter().map(|index| index.to_string()).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|item| format!("{}:{}", item.index, enc_page_diff(&item.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|item| format!("{}:{}", item.index, enc_page(&item.page))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_pages_diff(body: &str) -> Result<PdfPagesDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_section, modified_section, added_section] = three.as_slice() else { return Err(format!("pages diff: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_section)?, ',').into_iter().filter(|entry| !entry.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_section)?, ',')
        .into_iter()
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("pages modified: bad entry {entry:?}"))?;
            Ok(PdfPageModified { index: parse_usize(index)?, diff: dec_page_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_section)?, ',')
        .into_iter()
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let (index, rest) = entry.split_once(':').ok_or_else(|| format!("pages added: bad entry {entry:?}"))?;
            Ok(PdfPageAdded { index: parse_usize(index)?, page: dec_page(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(PdfPagesDiff { removed, modified, added })
}
//#endregion 🔖️TextCodec

//#region 🔖️BinaryCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, text: &str) {
    store::pack_rt::write_varint_u64(out, text.len() as u64);
    out.extend_from_slice(text.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let length = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
    String::from_utf8(reader.read_bytes(length).map_err(|error| error.to_string())?.to_vec()).map_err(|error| error.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_page_bin(page: &PageDoc, out: &mut Vec<u8>) {
    out.extend_from_slice(&page.width.to_le_bytes());
    out.extend_from_slice(&page.height.to_le_bytes());
    write_str_lp(out, &page.text);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_page_bin(reader: &mut store::ByteReader<'_>) -> Result<PageDoc, String> {
    let width = reader.read_f64_le().map_err(|error| error.to_string())?;
    let height = reader.read_f64_le().map_err(|error| error.to_string())?;
    Ok(PageDoc { width, height, text: read_str_lp(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_page_diff_bin(diff: &PdfPageDiff, out: &mut Vec<u8>) {
    out.push(u8::from(diff.width.is_some()));
    if let Some(value) = diff.width {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out.push(u8::from(diff.height.is_some()));
    if let Some(value) = diff.height {
        out.extend_from_slice(&value.to_le_bytes());
    }
    out.push(u8::from(diff.text.is_some()));
    if let Some(value) = &diff.text {
        write_str_lp(out, value);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_page_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfPageDiff, String> {
    let mut diff = PdfPageDiff::default();
    if reader.read_u8().map_err(|error| error.to_string())? != 0 {
        diff.width = Some(reader.read_f64_le().map_err(|error| error.to_string())?);
    }
    if reader.read_u8().map_err(|error| error.to_string())? != 0 {
        diff.height = Some(reader.read_f64_le().map_err(|error| error.to_string())?);
    }
    if reader.read_u8().map_err(|error| error.to_string())? != 0 {
        diff.text = Some(read_str_lp(reader)?);
    }
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_pages_diff_bin(diff: &PdfPagesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for index in &diff.removed {
        store::pack_rt::write_varint_u64(out, *index as u64);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for item in &diff.modified {
        store::pack_rt::write_varint_u64(out, item.index as u64);
        enc_page_diff_bin(&item.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for item in &diff.added {
        store::pack_rt::write_varint_u64(out, item.index as u64);
        enc_page_bin(&item.page, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_pages_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<PdfPagesDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|error| error.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
        modified.push(PdfPageModified { index, diff: dec_page_diff_bin(reader)? });
    }
    let added_count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
        added.push(PdfPageAdded { index, page: dec_page_bin(reader)? });
    }
    Ok(PdfPagesDiff { removed, modified, added })
}
//#endregion 🔖️BinaryCodec

//#region 🔖️DiffCodec
impl protocol::DiffCodec for PdfDiff {
    /// **Grammar**: `pages=<triple>` when the page lane moved, the empty string when nothing did —
    /// one space-separated `name=value` token per changed top-level field, exactly the convention
    /// the sibling 1.7 diff prints in.
    fn print_diff(&self) -> String {
        match &self.pages {
            Some(pages) => format!("pages={}", enc_pages_diff(pages)),
            None => String::new(),
        }
    }

    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        let parse = |line: &str| -> Result<Self, String> {
            let mut diff = PdfDiff::default();
            if line.is_empty() {
                return Ok(diff);
            }
            for token in line.split(' ') {
                match token.strip_prefix("pages=") {
                    Some(rest) => diff.pages = Some(dec_pages_diff(rest)?),
                    None => return Err(format!("pdf 1.4 diff: unknown token {token:?}")),
                }
            }
            Ok(diff)
        };
        parse(line).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
    }

    /// 🧪️ Real binary frame (`format u8 | flags u8 | [pages]`), matching
    /// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
    /// varint-counted, length-prefixed, genuinely structured, never `print_diff().into_bytes()`.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let flags: u8 = u8::from(self.pages.is_some());
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(pages) = &self.pages {
            enc_pages_diff_bin(pages, &mut out);
        }
        Ok(out)
    }

    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let format = reader.read_u8().map_err(|error| malformed("diff format", 0, error.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("diff format", 0, format!("expected {}, got {format}", store::pack_rt::OP_BINARY_FORMAT)));
        }
        let flags = reader.read_u8().map_err(|error| malformed("diff flags", 1, error.to_string()))?;
        if flags & !0b0000_0001 != 0 {
            return Err(malformed("diff flags", 1, format!("unknown flag bits {:#010b}", flags & !0b0000_0001)));
        }
        let pages = if flags & 0b0000_0001 != 0 { Some(dec_pages_diff_bin(&mut reader).map_err(|error| malformed("diff pages", reader.position(), error))?) } else { None };
        if reader.remaining() != 0 {
            return Err(malformed("diff trailing bytes", reader.position(), format!("{} trailing bytes", reader.remaining())));
        }
        Ok(PdfDiff { pages })
    }
}
//#endregion 🔖️DiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn page(width: f64, height: f64, text: &str) -> PageDoc {
        PageDoc { width, height, text: text.into() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snap(pages: Vec<PageDoc>) -> PdfSnapshot {
        PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), pages }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn one(width: f64, height: f64, text: &str) -> PdfSnapshot {
        snap(vec![page(width, height, text)])
    }

    //#region between_roundtrip_law
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = one(612.0, 792.0, "hello");
        let b = one(300.0, 400.0, "world");
        assert_eq!(PdfDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(PdfDiff::between(&b, &a).apply(&b).unwrap(), a);
        assert!(PdfDiff::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law_across_a_growing_and_shrinking_page_tree() {
        let one_page = snap(vec![page(612.0, 792.0, "a")]);
        let three_pages = snap(vec![page(612.0, 792.0, "a"), page(595.276, 841.89, "b"), page(200.0, 300.0, "")]);
        assert_eq!(PdfDiff::between(&one_page, &three_pages).apply(&one_page).unwrap(), three_pages);
        assert_eq!(PdfDiff::between(&three_pages, &one_page).apply(&three_pages).unwrap(), one_page);
    }
    //#endregion between_roundtrip_law

    //#region inverse_law
    #[semio_framework_async_macros::async_test]
    async fn inverse_law_diff_level() {
        let a = snap(vec![page(612.0, 792.0, "hello"), page(10.0, 20.0, "second")]);
        let b = snap(vec![page(300.0, 400.0, "world")]);
        let diff = PdfDiff::between(&a, &b);
        let mid = diff.apply(&a).unwrap();
        assert_eq!(mid, b);
        assert_eq!(diff.inverse(&a).apply(&mid).unwrap(), a);
    }
    //#endregion inverse_law

    //#region absorb_law
    #[semio_framework_async_macros::async_test]
    async fn absorb_law_sequential_composition() {
        let s0 = one(612.0, 792.0, "a");
        let s1 = one(300.0, 792.0, "a");
        let s2 = one(300.0, 400.0, "b");
        let d1 = PdfDiff::between(&s0, &s1);
        let d2 = PdfDiff::between(&s1, &s2);
        let sequential = d2.apply(&d1.apply(&s0).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&s0).unwrap(), sequential);
        assert_eq!(sequential, s2);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_sequential_composition_over_page_insertion_and_removal() {
        let s0 = snap(vec![page(1.0, 1.0, "a"), page(2.0, 2.0, "b")]);
        let s1 = snap(vec![page(1.0, 1.0, "a"), page(3.0, 3.0, "c"), page(2.0, 2.0, "b")]);
        let s2 = snap(vec![page(1.0, 1.0, "a"), page(3.0, 3.0, "c!")]);
        let d1 = PdfDiff::between(&s0, &s1);
        let d2 = PdfDiff::between(&s1, &s2);
        let sequential = d2.apply(&d1.apply(&s0).unwrap()).unwrap();
        assert_eq!(sequential, s2);
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&s0).unwrap(), sequential);
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_associativity() {
        let s0 = one(1.0, 1.0, "a");
        let s1 = one(2.0, 1.0, "a");
        let s2 = one(2.0, 2.0, "b");
        let s3 = one(3.0, 2.0, "c");
        let d1 = PdfDiff::between(&s0, &s1);
        let d2 = PdfDiff::between(&s1, &s2);
        let d3 = PdfDiff::between(&s2, &s3);
        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());
        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);
        assert_eq!(left.apply(&s0).unwrap(), s3);
        assert_eq!(right.apply(&s0).unwrap(), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law

    //#region validation
    #[semio_framework_async_macros::async_test]
    async fn a_removal_of_a_page_the_base_does_not_have_is_refused() {
        let base = one(612.0, 792.0, "a");
        let diff = PdfDiff { pages: Some(PdfPagesDiff { removed: vec![7], ..Default::default() }) };
        assert!(diff.apply(&base).is_err());
    }
    //#endregion validation

    //#region field_sweep
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> PdfSnapshot {
        one(612.0, 792.0, "base text")
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> PdfSnapshot {
        one(300.5, 400.25, "changed text")
    }

    #[semio_framework_async_macros::async_test]
    async fn field_sweep_between_roundtrips_both_directions() {
        let (a, b) = (sweep_a(), sweep_b());
        assert_eq!(PdfDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(PdfDiff::between(&b, &a).apply(&b).unwrap(), a);
        assert!(PdfDiff::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn field_sweep_every_field_present_in_diff() {
        let (a, b) = (sweep_a(), sweep_b());
        let diff = PdfDiff::between(&a, &b).pages.expect("the page lane moved");
        let page = &diff.modified.first().expect("page 0 is modified").diff;
        assert_eq!(page.width, Some(300.5));
        assert_eq!(page.height, Some(400.25));
        assert_eq!(page.text, Some("changed text".to_string()));
    }
    //#endregion field_sweep

    //#region diff_codec_text_binary_roundtrip_law
    /// 🧪️ `protocol::DiffCodec` LAW — exercises a modified page, an inserted page, a removed page
    /// and the fully-empty diff, both text (`print_diff`/`parse_diff`) and binary
    /// (`encode_diff`/`decode_diff`) sides.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;
        let (a, b) = (sweep_a(), sweep_b());
        let grown = snap(vec![page(612.0, 792.0, "base text"), page(1.0, 2.0, "added (with parens) and a comma,")]);
        let cases = vec![PdfDiff::between(&a, &b), PdfDiff::between(&a, &grown), PdfDiff::between(&grown, &a), PdfDiff::between(&a, &a)];
        for diff in cases {
            let printed = diff.print_diff();
            assert!(!printed.contains('\n'), "print_diff must not contain a newline: {printed:?}");
            let parsed = PdfDiff::parse_diff(&printed).expect("parse_diff must accept its own print_diff output");
            assert_eq!(parsed, diff, "parse_diff(print_diff(d)) must equal d");

            let encoded = diff.encode_diff().expect("encode_diff must succeed");
            let decoded = PdfDiff::decode_diff(&encoded).expect("decode_diff must accept its own encode_diff output");
            assert_eq!(decoded, diff, "decode_diff(encode_diff(d)) must equal d");
        }
    }
    //#endregion diff_codec_text_binary_roundtrip_law
}
//#endregion 🧪️Tests
