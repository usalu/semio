//! 🧱️ Variable length record diff of the LAS diff: the sparse record diff, its rows and the absorb and inverse of the record collection. See [`super`].

use super::*;

//#region 🔖️VlrDiff
/// 📦️ Sparse per-field patch for one `LasVlr`. `data` is retained/replaced byte-verbatim
/// (weak-value raw-retention field, never sub-diffed).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LasVlrDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<u16>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_vlr_diff(vlr: &mut LasVlr, diff: &LasVlrDiff) {
    if let Some(v) = &diff.user_id {
        vlr.user_id = v.clone();
    }
    if let Some(v) = diff.record_id {
        vlr.record_id = v;
    }
    if let Some(v) = &diff.description {
        vlr.description = v.clone();
    }
    if let Some(v) = &diff.data {
        vlr.data = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn vlr_inverse(diff: &LasVlrDiff, base: &LasVlr) -> LasVlrDiff {
    LasVlrDiff { user_id: diff.user_id.as_ref().map(|_| base.user_id.clone()), record_id: diff.record_id.map(|_| base.record_id), description: diff.description.as_ref().map(|_| base.description.clone()), data: diff.data.as_ref().map(|_| base.data.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_vlr_rows(base: &mut LasVlrDiff, other: LasVlrDiff) {
    if other.user_id.is_some() {
        base.user_id = other.user_id;
    }
    if other.record_id.is_some() {
        base.record_id = other.record_id;
    }
    if other.description.is_some() {
        base.description = other.description;
    }
    if other.data.is_some() {
        base.data = other.data;
    }
}

/// 📦️ One `vlrs.modified[]` entity — `index` is the VLR's position in BASE.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LasVlrModified {
    pub index: usize,
    pub diff: LasVlrDiff,
}

/// 📦️ One `vlrs.added[]` entity — `index` is the VLR's position in the FINAL sequence.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LasVlrAdded {
    pub index: usize,
    pub vlr: LasVlr,
}

/// 📦️ Sparse index-keyed `vlrs` triple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct LasVlrsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<LasVlrModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<LasVlrAdded>,
}

impl LasVlrsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    /// ▶️ Applies this triple: `modified` by BASE index (no-op if since-removed), then `removed`
    /// (descending order doesn't matter — collected as a set), then `added` ascending, clamped.
    pub(super) fn apply_unchecked(&self, base: &[LasVlr]) -> Vec<LasVlr> {
        let mut items = base.to_vec();
        for m in &self.modified {
            apply_vlr_diff(&mut items[m.index], &m.diff);
        }
        let mut removed = self.removed.clone();
        removed.sort_unstable_by(|a, b| b.cmp(a));
        for index in removed {
            items.remove(index);
        }
        let mut added = self.added.clone();
        added.sort_by_key(|a| a.index);
        for a in added {
            items.insert(a.index, a.vlr);
        }
        items
    }

    pub fn apply(&self, base: &[LasVlr]) -> MutationApplyResult<Vec<LasVlr>> {
        validate_indexed_targets(base.len(), &self.removed, self.modified.iter().map(|value| value.index), self.added.iter().map(|value| value.index), "vlrs")?;
        Ok(self.apply_unchecked(base))
    }

    /// ↩️ The negative triple against the base VLRs.
    pub(super) fn inverse(&self, base: &[LasVlr]) -> Self {
        let modified: Vec<(usize, LasVlrDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, LasVlr)> = self.added.iter().map(|a| (a.index, a.vlr.clone())).collect();
        let (removed, modified, added) = inverse_indexed_triple(&self.removed, &modified, &added, base, vlr_inverse);
        Self { removed, modified: modified.into_iter().map(|(index, diff)| LasVlrModified { index, diff }).collect(), added: added.into_iter().map(|(index, vlr)| LasVlrAdded { index, vlr }).collect() }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_vlrs(d1: Option<LasVlrsDiff>, d2: Option<LasVlrsDiff>) -> Option<LasVlrsDiff> {
    let (d1, d2) = match (d1, d2) {
        (None, None) => return None,
        (Some(d1), None) => return Some(d1),
        (None, Some(d2)) => return Some(d2),
        (Some(d1), Some(d2)) => (d1, d2),
    };
    let d1m: Vec<(usize, LasVlrDiff)> = d1.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let d1a: Vec<(usize, LasVlr)> = d1.added.iter().map(|a| (a.index, a.vlr.clone())).collect();
    let d2m: Vec<(usize, LasVlrDiff)> = d2.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
    let d2a: Vec<(usize, LasVlr)> = d2.added.iter().map(|a| (a.index, a.vlr.clone())).collect();
    let (removed, modified, added) = absorb_indexed_triple((&d1.removed, &d1m, &d1a), (&d2.removed, &d2m, &d2a), absorb_vlr_rows, apply_vlr_diff);
    let merged = LasVlrsDiff { removed, modified: modified.into_iter().map(|(index, diff)| LasVlrModified { index, diff }).collect(), added: added.into_iter().map(|(index, vlr)| LasVlrAdded { index, vlr }).collect() };
    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️VlrDiff
