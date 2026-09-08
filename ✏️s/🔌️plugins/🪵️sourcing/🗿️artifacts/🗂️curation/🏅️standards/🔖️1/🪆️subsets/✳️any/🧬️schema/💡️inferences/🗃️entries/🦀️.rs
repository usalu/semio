//! 🗃 `entries` — one named inference: a real census over the curation document's two lists —
//! `stockCount` (catalog size), `entryCount` (number of curated bill-of-quantities lines),
//! `totalCount` (sum of every curated line's `count` — the real total quantity picked). Whole-
//! snapshot scalar, not per-entity, so this leaf holds a plain pure function rather than an
//! `InferredField` chain — the family root's `impl protocol::Inference<CurationSnapshot>` calls it
//! directly.

use crate::CurationSnapshot;

//#region 🔖️Entries
/// 🗃️ Real census over `stock`/`curated`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CurationEntries {
    pub stock_count: u32,
    pub entry_count: u32,
    pub total_count: u32,
}

/// 🗃️ `stockCount` = `stock_extra.len()` (one entry per stock kind, 1:1 with the composed catalog's
/// `types` — reading the sourcing-owned overflow list directly avoids resolving the composed child
/// just to count it); `entryCount` = `curated.len()`; `totalCount` = sum of every curated line's
/// `count`.
pub fn compute_curation_entries(snapshot: &CurationSnapshot) -> CurationEntries {
    CurationEntries { stock_count: snapshot.stock_extra.len() as u32, entry_count: snapshot.curated.len() as u32, total_count: snapshot.curated.iter().map(|item| item.count).sum() }
}
//#endregion 🔖️Entries

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
