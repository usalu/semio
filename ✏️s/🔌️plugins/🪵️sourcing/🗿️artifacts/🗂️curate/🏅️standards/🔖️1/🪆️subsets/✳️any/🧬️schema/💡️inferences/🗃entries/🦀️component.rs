//! 🗃 `entries` — one named inference: a real census over the curate document's two lists —
//! `stockCount` (catalog size), `entryCount` (number of curated bill-of-quantities lines),
//! `totalCount` (sum of every curated line's `count` — the real total quantity picked). Whole-
//! snapshot scalar, not per-entity, so this leaf holds a plain pure function rather than an
//! `InferredField` chain — the family root's `impl protocol::Inference<CurateSnapshot>` calls it
//! directly.

use crate::artifacts::curate::CurateSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Entries
/// 🗃️ Real census over `stock`/`curated`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurateEntries {
    pub stock_count: u32,
    pub entry_count: u32,
    pub total_count: u32,
}

/// 🗃️ `stockCount` = `stock.len()`; `entryCount` = `curated.len()`; `totalCount` = sum of every
/// curated line's `count` — real derived quantities over the only two persistent lists this
/// document has.
pub fn compute_curate_entries(snapshot: &CurateSnapshot) -> CurateEntries {
    CurateEntries {
        stock_count: snapshot.stock.len() as u32,
        entry_count: snapshot.curated.len() as u32,
        total_count: snapshot.curated.iter().map(|item| item.count).sum(),
    }
}
//#endregion 🔖️Entries

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::curate::{CuratedItem, GeometryRecipe, ObjectKind};

    fn object_kind(id: &str) -> ObjectKind {
        ObjectKind { id: id.into(), name: id.into(), module_id: "beams".into(), typology_path: vec!["beams".into()], availability: 1, geometry: Box::new(GeometryRecipe::Box { width: 0.2, height: 0.4, depth: 6.0 }) }
    }

    #[test]
    fn empty_snapshot_yields_a_zero_census() {
        let entries = compute_curate_entries(&CurateSnapshot::default());
        assert_eq!(entries, CurateEntries::default());
    }

    #[test]
    fn stock_and_curated_lines_are_counted_exactly() {
        let snapshot = CurateSnapshot {
            stock: vec![object_kind("a"), object_kind("b"), object_kind("c")],
            curated: vec![CuratedItem { object_id: "a".into(), count: 5 }, CuratedItem { object_id: "b".into(), count: 3 }],
        };
        let entries = compute_curate_entries(&snapshot);
        assert_eq!(entries.stock_count, 3);
        assert_eq!(entries.entry_count, 2);
        assert_eq!(entries.total_count, 8);
    }

    #[test]
    fn entries_is_deterministic() {
        let snapshot = CurateSnapshot { stock: Vec::new(), curated: vec![CuratedItem { object_id: "a".into(), count: 1 }] };
        assert_eq!(compute_curate_entries(&snapshot), compute_curate_entries(&snapshot));
    }
}
//#endregion 🧪️Tests
