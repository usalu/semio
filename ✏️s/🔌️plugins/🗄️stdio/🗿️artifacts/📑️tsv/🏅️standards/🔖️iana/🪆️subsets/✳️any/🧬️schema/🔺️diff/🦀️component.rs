//! 🔺️ TsvDiff — handcrafted sparse structural diff. `records` is an index-keyed
//! removed/modified/added triple (TSV rows have no stable identity beyond position, same as
//! csv's own records); each modified row's own columns get a sparse positional patch via
//! [`TsvRowDiff`] (there is no insert-column/remove-column mutation — a patched position only
//! ever replaces an EXISTING cell — so a row's column count never resizes except via a whole-row
//! add/remove at the `records` collection level, matching csv's own `CsvRecordDiff` convention).

use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::{LineEnding, TsvSnapshot};
use protocol::command::DiffAlgebra;
use protocol::DiffCodec;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️RowDiff
/// 🔺️ Sparse diff for a single TSV row (`Vec<String>`) — positional per-column patch list,
/// `None` at a position means that column is unchanged.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TsvRowDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Option<String>>>,
}

impl TsvRowDiff {
    /// 🕳️ Whether this patch changes nothing.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        match &self.fields {
            None => true,
            Some(v) => v.iter().all(|f| f.is_none()),
        }
    }
    /// ▶️ Applies this patch to a row.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[String]) -> Vec<String> {
        match &self.fields {
            None => base.to_vec(),
            Some(patches) => {
                let mut row = base.to_vec();
                for (i, patch) in patches.iter().enumerate() {
                    if let Some(v) = patch {
                        if let Some(cell) = row.get_mut(i) {
                            *cell = v.clone();
                        }
                    }
                }
                row
            }
        }
    }
    /// 🧭️ State delta between two rows with the SAME column count (positional patch). Callers
    /// with differing column counts must instead express the change as a remove-then-add pair
    /// at the `records` collection level (see `TsvDiff::between`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(base: &[String], other: &[String]) -> Self {
        debug_assert_eq!(base.len(), other.len());
        let mut any = false;
        let patches: Vec<Option<String>> = base
            .iter()
            .zip(other.iter())
            .map(|(b, o)| {
                if b == o {
                    None
                } else {
                    any = true;
                    Some(o.clone())
                }
            })
            .collect();
        Self { fields: if any { Some(patches) } else { None } }
    }
    /// ➕️ Structural per-position absorb: `other`'s populated positions win.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        match (&mut self.fields, other.fields) {
            (_, None) => {}
            (slot @ None, Some(f2)) => *slot = Some(f2),
            (Some(f1), Some(f2)) => {
                if f2.len() > f1.len() {
                    f1.resize(f2.len(), None);
                }
                for (i, patch2) in f2.into_iter().enumerate() {
                    if patch2.is_some() {
                        f1[i] = patch2;
                    }
                }
            }
        }
    }
}
//#endregion 🔖️RowDiff

//#region 🔖️RecordsDiff
/// 🧩 One row patched-in-place at a BASE index.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TsvRowModified {
    pub index: usize,
    pub diff: TsvRowDiff,
}

/// 🧩 One row inserted at a FINAL index.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TsvRowAdded {
    pub index: usize,
    pub row: Vec<String>,
}

/// 🔺️ Index-keyed removed/modified/added triple over `TsvSnapshot::records`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TsvRowsDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<TsvRowModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<TsvRowAdded>,
}

impl TsvRowsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️RecordsDiff

//#region 🔖️IndexTransport
// 🧮 Base-free index transport for absorb — identical in shape to csv's own
// `simulate_slots`/`base_len_hint`/`absorb_records`.
#[derive(Clone, Copy, Debug)]
enum Slot {
    Base(usize),
    Added(usize),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn simulate_slots(len: usize, removed: &[usize], added_indices: &[usize]) -> Vec<Slot> {
    let mut slots: Vec<Slot> = (0..len).map(Slot::Base).collect();
    let mut removed_desc = removed.to_vec();
    removed_desc.sort_unstable_by(|a, b| b.cmp(a));
    removed_desc.dedup();
    for r in removed_desc {
        if r < slots.len() {
            slots.remove(r);
        }
    }
    let mut order: Vec<usize> = (0..added_indices.len()).collect();
    order.sort_by_key(|&i| added_indices[i]);
    for i in order {
        let at = added_indices[i].min(slots.len());
        slots.insert(at, Slot::Added(i));
    }
    slots
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn base_len_hint(removed: &[usize], modified_indices: impl Iterator<Item = usize>, added_indices: impl Iterator<Item = usize>) -> usize {
    removed.iter().copied().chain(modified_indices).chain(added_indices).max().map(|m| m + 1).unwrap_or(0)
}
//#endregion 🔖️IndexTransport

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.tsv`. No `snapshot: Option<TsvSnapshot>` full-replace slot — even
/// `SetSnapshot`'s diff is `TsvDiff::between(base, next)`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tsv.diff")]
pub struct TsvDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<bool>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub records: Option<TsvRowsDiff>,
}

impl MutationDiff<TsvSnapshot> for TsvDiff {
    fn apply(&self, base: &TsvSnapshot) -> MutationApplyResult<TsvSnapshot> {
        validate_tsv_diff(self, base)?;
        Ok(apply_tsv_diff_unchecked(self, base))
    }

    fn absorb(&mut self, other: Self) {
        if other.trailing_newline.is_some() {
            self.trailing_newline = other.trailing_newline;
        }
        if other.line_ending.is_some() {
            self.line_ending = other.line_ending;
        }
        let d2 = match other.records {
            None => return,
            Some(d2) => d2,
        };
        let d1 = match self.records.take() {
            None => {
                self.records = Some(d2);
                return;
            }
            Some(d1) => d1,
        };
        self.records = Some(absorb_records(d1, d2));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_tsv_diff(diff: &TsvDiff, base: &TsvSnapshot) -> MutationApplyResult<()> {
    let Some(records) = &diff.records else { return Ok(()) };
    let mut removed = std::collections::HashSet::new();
    for &index in &records.removed {
        if index >= base.records.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "row removal target does not exist"));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "row removal target is repeated"));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &records.modified {
        if entry.index >= base.records.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "row modification target does not exist"));
        }
        if removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "row modification targets a removed item"));
        }
        if !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "row modification target is repeated"));
        }
        if let Some(fields) = &entry.diff.fields {
            if fields.len() > base.records[entry.index].len() {
                return Err(MutationApplyError::new("mutation.apply.invalid-index", "row field patch exceeds the base row"));
            }
        }
    }
    let final_len = base.records.len() - removed.len() + records.added.len();
    let mut added = std::collections::HashSet::new();
    for entry in &records.added {
        if entry.index > final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "row addition is outside the final collection"));
        }
        if !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "row addition occupies a repeated final position"));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_tsv_diff_unchecked(diff: &TsvDiff, base: &TsvSnapshot) -> TsvSnapshot {
    let mut next = base.clone();
    if let Some(v) = diff.trailing_newline {
        next.trailing_newline = v;
    }
    if let Some(v) = diff.line_ending {
        next.line_ending = v;
    }
    if let Some(rdiff) = &diff.records {
        for m in &rdiff.modified {
            if let Some(row) = next.records.get_mut(m.index) {
                *row = m.diff.apply(row);
            }
        }
        let mut removed_desc = rdiff.removed.clone();
        removed_desc.sort_unstable_by(|a, b| b.cmp(a));
        removed_desc.dedup();
        for idx in removed_desc {
            if idx < next.records.len() {
                next.records.remove(idx);
            }
        }
        let mut added_asc = rdiff.added.clone();
        added_asc.sort_by_key(|a| a.index);
        for a in added_asc {
            let at = a.index.min(next.records.len());
            next.records.insert(at, a.row);
        }
    }
    next
}

/// ➕️ Structural, total, base-free absorb of two `records` triples (same algorithm as csv's).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_records(d1: TsvRowsDiff, d2: TsvRowsDiff) -> TsvRowsDiff {
    let d1_added_indices: Vec<usize> = d1.added.iter().map(|a| a.index).collect();
    let removed_count = {
        let mut r = d1.removed.clone();
        r.sort_unstable();
        r.dedup();
        r.len()
    };
    let needed_mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0);
    let base_len = base_len_hint(&d1.removed, d1.modified.iter().map(|m| m.index), d1_added_indices.iter().copied()).max((needed_mid_len + removed_count).saturating_sub(d1.added.len()));
    let mid_slots = simulate_slots(base_len, &d1.removed, &d1_added_indices);

    let mut final_removed: Vec<usize> = d1.removed.clone();
    let mut modified_map: BTreeMap<usize, TsvRowDiff> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
    let mut added_alive: Vec<Option<TsvRowAdded>> = d1.added.into_iter().map(Some).collect();

    for mid_idx in &d2.removed {
        match mid_slots.get(*mid_idx) {
            Some(Slot::Base(b)) => {
                final_removed.push(*b);
                modified_map.remove(b);
            }
            Some(Slot::Added(ai)) => {
                added_alive[*ai] = None;
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid_slots.get(m2.index) {
            Some(Slot::Base(b)) => {
                modified_map.entry(*b).or_default().absorb(m2.diff.clone());
            }
            Some(Slot::Added(ai)) => {
                if let Some(added) = added_alive[*ai].as_mut() {
                    added.row = m2.diff.apply(&added.row);
                }
            }
            None => {}
        }
    }

    final_removed.sort_unstable();
    final_removed.dedup();
    for r in &final_removed {
        modified_map.remove(r);
    }
    let mut final_modified: Vec<TsvRowModified> = modified_map.into_iter().filter(|(_, d)| !d.is_empty()).map(|(index, diff)| TsvRowModified { index, diff }).collect();
    final_modified.sort_by_key(|m| m.index);

    let alive_mid_positions: Vec<usize> = mid_slots
        .iter()
        .enumerate()
        .filter_map(|(pos, slot)| match slot {
            Slot::Added(ai) if added_alive[*ai].is_some() => Some(pos),
            _ => None,
        })
        .collect();
    let d2_added_indices: Vec<usize> = d2.added.iter().map(|a| a.index).collect();
    let mid_len = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).chain(alive_mid_positions.iter().copied()).chain(d2_added_indices.iter().copied()).max().map(|m| m + 1).unwrap_or(0);
    let after_slots = simulate_slots(mid_len, &d2.removed, &d2_added_indices);
    let mut mid_to_after: HashMap<usize, usize> = HashMap::new();
    for (pos, slot) in after_slots.iter().enumerate() {
        if let Slot::Base(m) = slot {
            mid_to_after.insert(*m, pos);
        }
    }

    let mut final_added: Vec<TsvRowAdded> = Vec::new();
    for (ai, alive) in added_alive.into_iter().enumerate() {
        if let Some(added) = alive {
            let mid_pos = mid_slots.iter().position(|s| matches!(s, Slot::Added(idx) if *idx == ai)).expect("added_alive index always has a corresponding mid slot");
            if let Some(after_pos) = mid_to_after.get(&mid_pos) {
                final_added.push(TsvRowAdded { index: *after_pos, row: added.row });
            }
        }
    }
    for a2 in d2.added {
        final_added.push(a2);
    }
    final_added.sort_by_key(|a| a.index);

    TsvRowsDiff { removed: final_removed, modified: final_modified, added: final_added }
}

impl DiffAlgebra<TsvSnapshot> for TsvDiff {
    fn inverse(&self, base: &TsvSnapshot) -> Self {
        let applied = apply_tsv_diff_unchecked(self, base);
        Self::between(&applied, base)
    }

    fn between(base: &TsvSnapshot, other: &TsvSnapshot) -> Self {
        let trailing_newline = (base.trailing_newline != other.trailing_newline).then_some(other.trailing_newline);
        let line_ending = (base.line_ending != other.line_ending).then_some(other.line_ending);

        let mut removed = Vec::new();
        let mut modified = Vec::new();
        let mut added = Vec::new();
        let min_len = base.records.len().min(other.records.len());
        for i in 0..min_len {
            let b = &base.records[i];
            let o = &other.records[i];
            if b == o {
                continue;
            }
            if b.len() == o.len() {
                let d = TsvRowDiff::between(b, o);
                if !d.is_empty() {
                    modified.push(TsvRowModified { index: i, diff: d });
                }
            } else {
                removed.push(i);
                added.push(TsvRowAdded { index: i, row: o.clone() });
            }
        }
        for i in min_len..base.records.len() {
            removed.push(i);
        }
        for i in min_len..other.records.len() {
            added.push(TsvRowAdded { index: i, row: other.records[i].clone() });
        }

        let records = if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(TsvRowsDiff { removed, modified, added }) };
        Self { trailing_newline, line_ending, records }
    }

    fn is_empty(&self) -> bool {
        self.trailing_newline.is_none() && self.line_ending.is_none() && self.records.as_ref().map_or(true, TsvRowsDiff::is_empty)
    }
}

/// 🧩 Builds a set-snapshot diff (sparse field-by-field delta, never a full-replace slot).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &TsvSnapshot, next: &TsvSnapshot) -> TsvDiff {
    TsvDiff::between(base, next)
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` — `TsvRowDiff::fields: Option<Vec<Option<String>>>`
/// hits the same dsl-derive `Vec<Option<T>>` rejection csv's `CsvRecordDiff` documents (one
/// `Vec`-wrapped tri-state layer). **Grammar**: one space-separated `name=value` token per
/// changed top-level field; `records` prints as `records{[removed];[modified];[added]}`. Strings
/// are lowercase hex (TSV cells legally contain almost anything except tab/CR/LF, which this
/// grammar's own separators are built from — hex sidesteps escaping entirely).
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
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
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

//#region 🔖️ValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_row(r: &[String]) -> String {
    format!("[{}]", r.iter().map(|f| enc_str(f)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_row(s: &str) -> Result<Vec<String>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_str).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_line_ending(l: LineEnding) -> &'static str {
    match l {
        LineEnding::Lf => "lf",
        LineEnding::Crlf => "crlf",
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_line_ending(s: &str) -> Result<LineEnding, String> {
    match s {
        "lf" => Ok(LineEnding::Lf),
        "crlf" => Ok(LineEnding::Crlf),
        other => Err(format!("line ending: unknown value {other:?}")),
    }
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_row_diff(d: &TsvRowDiff) -> String {
    encode_option(&d.fields, |fields| format!("[{}]", fields.iter().map(|f| encode_option(f, |v| enc_str(v))).collect::<Vec<_>>().join(",")))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_row_diff(s: &str) -> Result<TsvRowDiff, String> {
    let fields = decode_option(s, |inner| split_top_level(strip_brackets(inner)?, ',').into_iter().filter(|s| !s.is_empty()).map(|p| decode_option(p, dec_str)).collect::<Result<Vec<_>, String>>())?;
    Ok(TsvRowDiff { fields })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_records_diff(d: &TsvRowsDiff) -> String {
    let removed = d.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = d.modified.iter().map(|m| format!("{}:{}", m.index, enc_row_diff(&m.diff))).collect::<Vec<_>>().join(",");
    let added = d.added.iter().map(|a| format!("{}:{}", a.index, enc_row(&a.row))).collect::<Vec<_>>().join(",");
    format!("records{{[{removed}];[{modified}];[{added}]}}")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_records_diff(body: &str) -> Result<TsvRowsDiff, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("records: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("records modified: bad entry {entry:?}"))?;
            Ok(TsvRowModified { index: parse_usize(idx)?, diff: dec_row_diff(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("records added: bad entry {entry:?}"))?;
            Ok(TsvRowAdded { index: parse_usize(idx)?, row: dec_row(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(TsvRowsDiff { removed, modified, added })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_tsv_diff(d: &TsvDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = d.trailing_newline {
        tokens.push(format!("trailing-newline={}", if v { 1 } else { 0 }));
    }
    if let Some(v) = d.line_ending {
        tokens.push(format!("line-ending={}", enc_line_ending(v)));
    }
    if let Some(v) = &d.records {
        tokens.push(enc_records_diff(v));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_tsv_diff(line: &str) -> Result<TsvDiff, String> {
    let mut d = TsvDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("trailing-newline=") {
            d.trailing_newline = Some(rest == "1");
        } else if let Some(rest) = token.strip_prefix("line-ending=") {
            d.line_ending = Some(dec_line_ending(rest)?);
        } else if let Some(rest) = token.strip_prefix("records{") {
            d.records = Some(dec_records_diff(rest.strip_suffix('}').ok_or_else(|| "records: missing closing brace".to_string())?)?);
        } else {
            return Err(format!("tsv diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl DiffCodec for TsvDiff {
    fn print_diff(&self) -> String {
        print_tsv_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_tsv_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim, same simplification csv's/gif89a's hand-rolled
    /// `DiffCodec`s use.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🧪️Tests
#[cfg(test)]
mod handcrafted_diff_codec_tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn row(fields: &[&str]) -> Vec<String> {
        fields.iter().map(|s| s.to_string()).collect()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot(records: Vec<Vec<String>>, trailing_newline: bool) -> TsvSnapshot {
        TsvSnapshot { records, trailing_newline, ..TsvSnapshot::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        let a = snapshot(vec![row(&["id", "name"]), row(&["1", "Oak"]), row(&["2", "Steel"])], true);
        let mut b = snapshot(vec![row(&["id", "name"]), row(&["1", "Oak, tricky [value]"]), row(&["2", "Steel"])], false);
        b.records.push(row(&["3", "new"]));
        let cases = vec![TsvDiff::default(), TsvDiff::between(&a, &b), TsvDiff::between(&b, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = TsvDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = TsvDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
